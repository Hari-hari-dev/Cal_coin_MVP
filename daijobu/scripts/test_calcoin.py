import asyncio
from pathlib import Path
import traceback
import json

# anchorpy
from anchorpy import Program, Provider, Wallet, Idl, Context
from anchorpy.program.namespace.instruction import AccountMeta

# Solana / Solders
from solders.keypair import Keypair
from solders.pubkey import Pubkey
from solders.system_program import ID as SYS_PROGRAM_ID
from solana.rpc.async_api import AsyncClient
from solana.rpc.commitment import Confirmed
from solana.rpc.core import RPCException

# For demonstration, we’ll assume these are pre-installed
SPL_TOKEN_PROGRAM_ID = Pubkey.from_string("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")
ASSOCIATED_TOKEN_PROGRAM_ID = Pubkey.from_string("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL")

# --------------------------------------------------------------------------
# EXAMPLE: Interacting with daily_facescan
# --------------------------------------------------------------------------
def derive_ata(owner: Pubkey, mint: Pubkey) -> Pubkey:
    """
    Derives the Associated Token Account (ATA) for (mint, owner).
    This matches the same seeds Anchor uses for:
      #[account(
         associated_token::mint = mint,
         associated_token::authority = owner
      )]
    Seeds = [owner, token_program_id, mint], with the associated_token_program as the PD program.
    """
    seeds = [
        bytes(owner),
        bytes(SPL_TOKEN_PROGRAM_ID),
        bytes(mint)
    ]
    (ata, _bump) = Pubkey.find_program_address(
        seeds,
        ASSOCIATED_TOKEN_PROGRAM_ID
    )
    return ata
async def anchorpy_demo():
    try:
        ##############################
        # 1) Setup Client + Program
        ##############################
        client = AsyncClient("https://api.devnet.solana.com", commitment=Confirmed)
        wallet = Wallet.local()  # uses ~/.config/solana/id.json by default
        provider = Provider(client, wallet)

        # Load IDL
        idl_path = Path("../target/idl/daily_facescan.json")
        if not idl_path.exists():
            print(f"IDL file not found at {idl_path.resolve()}")
            return

        with idl_path.open() as f:
            idl_json = f.read()

        # The Program ID from declare_id! in lib.rs
        program_id = Pubkey.from_string("BSPMVPbq78vJAYSma7abTjrKqUNshq4J1kxbWwcGXxdV")
        idl = Idl.from_json(idl_json)
        program = Program(idl, program_id, provider)
        print("Program loaded successfully.")

        ##############################
        # 2) Derive Keypairs/PDA
        ##############################
        # a) Airdrop Keypair (not a PDA). We'll "init" it in the 'initialize' instruction
        airdrop_kp = Keypair()
        print(f"[DEBUG] Airdrop key: {airdrop_kp.pubkey()}")

        # b) Mint Keypair (not a PDA). We'll init this in 'initialize' as well
        mint_kp = Keypair()
        print(f"[DEBUG] Mint key: {mint_kp.pubkey()}")

        # c) Derive the mint_authority PDA: seeds=[airdrop, "mint_authority"]
        (mint_authority_pda, bump) = Pubkey.find_program_address(
            [bytes(airdrop_kp.pubkey()), b"mint_authority"],
            program_id
        )
        print(f"[DEBUG] MintAuthority: {mint_authority_pda}, Bump={bump}")

        ##############################
        # 3) Call "initialize"
        ##############################
        # This creates the airdrop + mint on-chain with 9 decimals, daily_amount=1440, etc.
        # The code in lib.rs expects:
        #   initialize(ctx):
        #     airdrop, mint, mint_authority, authority, system_program, token_program, rent
        recipient_ata = derive_ata(wallet.public_key, mint_kp.pubkey())
    # The "recipient" public key

        try:
            tx_sig = await program.rpc["initialize"](
                ctx=Context(
                    accounts={
                        "airdrop":         airdrop_kp.pubkey(),
                        "mint":            mint_kp.pubkey(),
                        "mint_authority":  mint_authority_pda,
                        "authority":       wallet.public_key,
                        "system_program":  SYS_PROGRAM_ID,
                        "token_program":   SPL_TOKEN_PROGRAM_ID,
                        "rent": Pubkey.from_string("SysvarRent111111111111111111111111111111111"),
                    },
                    signers=[wallet.payer, airdrop_kp, mint_kp],  # we must sign for the new accounts
                )
            )
            print(f"[initialize] success! Tx: {tx_sig}")
        except RPCException as e:
            print(f"[initialize] RPCException: {e}")
            traceback.print_exc()

        ##############################
        # 4) Example: Claim
        ##############################
        # Usually, the user calls "claim" to get their daily tokens.
        # We'll do a short demonstration with a "fake" gateway token.
        gateway_token_kp = Keypair()  # In reality, you'd get a real gateway token or pass the correct pubkey
        payer_kp = wallet.payer       # The same wallet or a different user?

        # We also need a "ticket" account, seeds=[airdrop, recipient, "ticket_seed"]. We'll derive that.
        (ticket_pda, ticket_bump) = Pubkey.find_program_address(
            [bytes(airdrop_kp.pubkey()), bytes(wallet.public_key), b"ticket"],
            program_id
        )
        print(f"[DEBUG] Derived ticket PDA = {ticket_pda}, bump={ticket_bump}")

        # The "recipient" is the user who gets tokens. Let's assume wallet.public_key.
        # The code expects an associated token account for "recipient_token_account".
        # We'll guess it is derived automatically by "associated_token::authority=recipient".
        # But let's confirm or create it ourselves if needed.
        # For demonstration, let's skip explicit creation & rely on "init_if_needed" in the program.

        try:
            tx_sig = await program.rpc["claim"](
                ctx=Context(
                    accounts={
                        "airdrop":                airdrop_kp.pubkey(),
                        "payer":                  wallet.public_key,
                        "mint_authority":         mint_authority_pda,
                        "ticket":                 ticket_pda,
                        "mint":                   mint_kp.pubkey(),
                        "recipient_token_account": recipient_ata,  # let the program do "init_if_needed"
                        "gateway_token":          gateway_token_kp.pubkey(),
                        "recipient":              wallet.public_key,
                        "rent": Pubkey.from_string("SysvarRent111111111111111111111111111111111"),
                        "token_program":          SPL_TOKEN_PROGRAM_ID,
                        "system_program":         SYS_PROGRAM_ID,
                        "associated_token_program": ASSOCIATED_TOKEN_PROGRAM_ID,
                    },
                    signers=[wallet.payer],  # only the payer signs
                )
            )
            print(f"[claim] success! Tx: {tx_sig}")
        except RPCException as e:
            print(f"[claim] RPCException: {e}")
            traceback.print_exc()

        ##############################
        # 5) Add a new owner
        ##############################
        # In the default code, we have co_owners or owners. The "is_authorized" check
        # allows the main authority or co-owners to add an owner. We'll do an example:
        new_owner_kp = Keypair()
        print(f"[DEBUG] Attempting to add new owner: {new_owner_kp.pubkey()}")

        try:
            tx_sig = await program.rpc["add_owner"](
                new_owner_kp.pubkey(),
                ctx=Context(
                    accounts={
                        "airdrop": airdrop_kp.pubkey(),
                        "signer":  wallet.public_key,
                    },
                    signers=[wallet.payer],
                )
            )
            print(f"[add_owner] success! Tx: {tx_sig}")
        except RPCException as e:
            print(f"[add_owner] RPCException: {e}")
            traceback.print_exc()

        ##############################
        # 6) Remove that new owner
        ##############################
        # Example. If we want to remove the newly added key
        try:
            tx_sig = await program.rpc["delete_owner"](
                new_owner_kp.pubkey(),
                ctx=Context(
                    accounts={
                        "airdrop": airdrop_kp.pubkey(),
                        "signer":  wallet.public_key,
                    },
                    signers=[wallet.payer],
                )
            )
            print(f"[delete_owner] success! Tx: {tx_sig}")
        except RPCException as e:
            print(f"[delete_owner] RPCException: {e}")
            traceback.print_exc()

        ##############################
        # 7) Change the Gateway Network
        ##############################
        # Suppose we want to set a new gatekeeper pubkey.
        new_gatekeeper_pubkey = Pubkey.from_string("6FhraHnUCBLHUXyKypmZPv67VbbWb1GtPzmyrGLsZ7EV")
        try:
            tx_sig = await program.rpc["change_gateway_network"](
                new_gatekeeper_pubkey,
                ctx=Context(
                    accounts={
                        "airdrop": airdrop_kp.pubkey(),
                        "signer":  wallet.public_key,
                    },
                    signers=[wallet.payer],
                )
            )
            print(f"[change_gateway_network] success! Tx: {tx_sig}")
        except RPCException as e:
            print(f"[change_gateway_network] RPCException: {e}")
            traceback.print_exc()

        print("\nAll demonstration calls completed successfully.")
    except Exception as e:
        print(f"An unexpected error occurred: {e}")
        traceback.print_exc()
    finally:
        await client.close()
        print("Closed Solana RPC client.")


if __name__ == "__main__":
    asyncio.run(anchorpy_demo())
