import asyncio
import socket
import struct
import a2s
import re
import json
import traceback
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor

# Anchor / Solana
from solders.keypair import Keypair
from solders.pubkey import Pubkey
from solders.system_program import ID as SYS_PROGRAM_ID
from solana.rpc.async_api import AsyncClient
from solana.rpc.commitment import Confirmed
from solana.rpc.core import RPCException
from anchorpy import Program, Provider, Wallet, Idl, Context
from anchorpy.program.namespace.instruction import AccountMeta

###############################################################################
# Globals set after program setup
###############################################################################
program = None
provider = None
program_id = None
game_pda = None
dapp_pda = None

CHUNK_SIZE = 16  # how many players to mint per TX

###############################################################################
# 1) Load Validator Keypair
###############################################################################
def load_validator_keypair(filename="val1-keypair.json") -> Keypair:
    """Load from raw 64-byte secret in a JSON array, e.g. [12,34,56,...]."""
    def load_keypair(path: str) -> Keypair:
        with Path(path).open() as f:
            secret = json.load(f)
        return Keypair.from_bytes(bytes(secret[0:64]))
    return load_keypair(filename)

###############################################################################
# 2) Debug-check DApp
###############################################################################
async def debug_check_dapp_pda():
    dapp_data = await program.account["DApp"].fetch(dapp_pda)
    print("[DEBUG] DApp Account Data:")
    print(f"         owner                = {dapp_data.owner}")
    print(f"         global_player_count = {dapp_data.global_player_count}")
    print(f"         mint_pubkey         = {dapp_data.mint_pubkey}")

###############################################################################
# 3) Off-chain: fetch all PlayerPda => build (name -> index, name -> pda)
###############################################################################
async def fetch_player_pdas_map() -> dict:
    """
    Returns a dict:
      {
         "<player_name>": {
            "index":  <u32 index in the anchor code>,
            "pda":    <Pubkey for PlayerPda>
         },
         ...
      }
    
    We assume the "index" is just the ascending order that was used at creation time:
       seeds=[b"player_pda", dapp.global_player_count.to_le_bytes()]
    so the earliest is 0, then 1, etc.
    We'll do a naive approach: sort by creation or just rely on Anchor's .all() order.
    """
    all_records = await program.account["PlayerPda"].all()
    # We'll sort them by the order they appear in `dapp.global_player_count`.
    # If your code increments `dapp.global_player_count` each time, the Nth created is index=N-1
    # So we can track them in ascending order. Or you might store the actual "index" on the account.

    # We'll do: for each record, re-derive the index by comparing seeds. This is a trick:
    #   seeds: [b"player_pda", <u32 in little-endian>]
    # We can try every index up to global_player_count, or we can parse from the actual address. 
    # For simplicity, let's do a loop up to global_player_count and see which address matches each record.

    # Step A: fetch the dapp to see how many total players
    dapp_data = await program.account["DApp"].fetch(dapp_pda)
    total_count = dapp_data.global_player_count

    # Build a quick lookup: (pubkey_str -> index)
    pda_to_index = {}
    for i in range(total_count):
        seed_index_bytes = i.to_bytes(4, "little")
        (pda, _) = Pubkey.find_program_address([b"player_pda", seed_index_bytes], program.program_id)
        pda_to_index[str(pda)] = i

    # Now we build result => name => {index, pda}
    name_map = {}
    for rec in all_records:
        pkey_str = str(rec.public_key)
        # find the index
        if pkey_str in pda_to_index:
            real_idx = pda_to_index[pkey_str]
            player_name = rec.account.name
            name_map[player_name] = {
                "index": real_idx,
                "pda": rec.public_key
            }
        else:
            # This can happen if the record is from some older logic.
            pass

    return name_map

###############################################################################
# 4) Actually submit_minting_list on-chain in CHUNK_SIZE chunks
###############################################################################
async def submit_minting_list_with_leftover(
    game_number: int,
    matched_names: list[str],
    name_map: dict,
):
    """
    This breaks the matched names into chunks of CHUNK_SIZE (16).
    For each chunk, we:
     1) Build numeric player_ids (u32),
     2) Build leftover [PlayerPda, ATA, PlayerPda, ATA, ...],
     3) Call `submit_minting_list`.
    """
    validator_kp = load_validator_keypair()
    validator_pubkey = validator_kp.pubkey()

    # Derive the validator_pda
    seeds_val = [b"validator", game_number.to_bytes(4, "little"), bytes(validator_pubkey)]
    (validator_pda, _) = Pubkey.find_program_address(seeds_val, program.program_id)

    # We'll also fetch the DApp to see .mint_pubkey (fancy_mint) + we can guess the mint_auth
    dapp_data = await program.account["DApp"].fetch(dapp_pda)
    fancy_mint_pk = dapp_data.mint_pubkey

    # Derive mint_authority
    (mint_auth_pda, _) = Pubkey.find_program_address([b"mint_authority"], program_id)

    # Let’s chunk
    for start_idx in range(0, len(matched_names), CHUNK_SIZE):
        chunk = matched_names[start_idx : start_idx + CHUNK_SIZE]
        # Build leftover accounts
        leftover_accounts = []
        numeric_ids = []

        for name in chunk:
            # Look up the player's index + pda
            entry = name_map.get(name)
            if entry is None:
                print(f"Skipping name={name}, not found in name_map.")
                continue

            pid = entry["index"]
            player_pda_pubkey = entry["pda"]

            # We'll just do a "dummy ATA" for the demonstration
            # In production, you'd do an Associated Token Address:
            #   from spl.token.instructions import get_associated_token_address
            #   player_ata = get_associated_token_address(<actual player authority>, fancy_mint_pk)
            # Here:
            dummy_seed = f"dummyATA_{pid}".encode("utf-8")
            (dummy_ata_pubkey, _) = Pubkey.find_program_address([dummy_seed], program.program_id)

            leftover_accounts.append(
                AccountMeta(pubkey=player_pda_pubkey, is_signer=False, is_writable=True)
            )
            leftover_accounts.append(
                AccountMeta(pubkey=dummy_ata_pubkey, is_signer=False, is_writable=True)
            )
            numeric_ids.append(pid)

        if not numeric_ids:
            print("[DEBUG] This chunk is empty, skipping.")
            continue

        print(f"[DEBUG] Submitting chunk => {numeric_ids}")

        # Build anchorpy Context
        ctx = Context(
            accounts={
                "game":           game_pda,
                "validator_pda":  validator_pda,
                "validator":      validator_pubkey,
                "fancy_mint":     fancy_mint_pk,
                "dapp":           dapp_pda,
                "mint_authority": mint_auth_pda,
                "token_program":  Pubkey.from_string("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            },
            signers=[validator_kp],
            remaining_accounts=leftover_accounts
        )

        # Actually call the instruction
        try:
            tx_sig = await program.rpc["submit_minting_list"](
                game_number,
                numeric_ids,
                ctx=ctx
            )
            print(f"[INFO] submit_minting_list TX => {tx_sig}")

            # Optionally fetch logs
            confirmed_tx = await program.provider.connection.get_transaction(tx_sig, encoding="json")
            if confirmed_tx.value and confirmed_tx.value.transaction.meta:
                logs = confirmed_tx.value.transaction.meta.log_messages or []
                print("[DEBUG] Logs from this chunk's TX:")
                for line in logs:
                    print("   ", line)
            else:
                print("[DEBUG] No logs or missing transaction meta.")

        except RPCException as exc:
            print(f"[ERROR] chunk submission => {exc}")

        # Sleep a tiny bit if you want to throttle
        await asyncio.sleep(0.2)

###############################################################################
# 5) The main entrypoint
###############################################################################
async def main():
    try:
        print("Setting up provider and loading program IDL...")
        client = AsyncClient("http://localhost:8899", commitment=Confirmed)
        wallet = Wallet.local()

        global provider, program, program_id, dapp_pda, game_pda
        provider = Provider(client, wallet)

        # 1) Load the IDL
        idl_path = Path("../target/idl/fancoin.json")
        if not idl_path.exists():
            print(f"[ERROR] IDL file not found at {idl_path.resolve()}")
            return

        with idl_path.open() as f:
            idl_json = f.read()

        program_id = Pubkey.from_string("HP9ucKGU9Sad7EaWjrGULC2ZSyYD1ScxVPh15QmdRmut")
        idl = Idl.from_json(idl_json)
        program = Program(idl, program_id, provider)
        print("Program loaded successfully.")

        # 2) Derive PDAs
        game_number = 1
        (game_pda, _) = Pubkey.find_program_address([b"game", game_number.to_bytes(4, "little")], program_id)
        (dapp_pda, _) = Pubkey.find_program_address([b"dapp"], program_id)
        print(f"[DEBUG] using game_pda={game_pda}")
        print(f"[DEBUG] using dapp_pda={dapp_pda}")

        # Debug
        await debug_check_dapp_pda()

        # 3) Suppose you do your TFC server logic => you get a local dict of names => ephemeral keys
        #    We'll just do a placeholder matched list:
        local_server_names = [
            "Cherrybomb","Cheryl","Moose","No","oneredflag","Medic","choad","Bobbolo",
            "SmellULater","Bradley1968","... etc ..."  # truncated
        ]

        # 4) Fetch all on-chain PlayerPda => build name->index->pda
        name_map = await fetch_player_pdas_map()

        # 5) Intersection
        matched_names = list(set(local_server_names).intersection(set(name_map.keys())))
        print("[DEBUG] matched_names =>", matched_names)

        if not matched_names:
            print("No matched players found, done.")
            return

        # 6) Submit them in multiple chunks
        await submit_minting_list_with_leftover(game_number, matched_names, name_map)

    except Exception as e:
        print(f"[ERROR] Unexpected error:\n{e}")
        traceback.print_exc()
    finally:
        if provider and provider.connection:
            await provider.connection.close()
        print("Closed Solana RPC client.")

if __name__ == "__main__":
    asyncio.run(main())
