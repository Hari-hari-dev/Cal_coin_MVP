import asyncio
from pathlib import Path
from enum import IntEnum
import traceback
import json

# anchorpy
from anchorpy import Program, Provider, Wallet, Idl, Context
from anchorpy.program.namespace.instruction import AccountMeta

# solana / solders
from solders.keypair import Keypair
from solders.pubkey import Pubkey
from solders.system_program import ID as SYS_PROGRAM_ID
from solana.rpc.async_api import AsyncClient
from solana.rpc.commitment import Confirmed
from solana.rpc.core import RPCException

# We define the SPL token program ID manually
SPL_TOKEN_PROGRAM_ID = Pubkey.from_string("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")

class GameStatus(IntEnum):
    Probationary = 0
    Whitelisted = 1
    Blacklisted = 2


async def initialize_dapp(program: Program, client: AsyncClient) -> Pubkey:
    """Initialize the DApp, returning the DApp PDA."""
    print("Initializing DApp account...")
    dapp_pda, dapp_bump = Pubkey.find_program_address([b"dapp"], program.program_id)
    print(f"DApp PDA: {dapp_pda}, Bump: {dapp_bump}")

    # Check if DApp account already exists
    dapp_account = await client.get_account_info(dapp_pda, commitment=Confirmed)
    if dapp_account.value is None:
        try:
            tx = await program.rpc["initialize_dapp"](
                ctx=Context(
                    accounts={
                        "dapp": dapp_pda,
                        "user": program.provider.wallet.public_key,
                        "system_program": SYS_PROGRAM_ID,
                    },
                    signers=[program.provider.wallet.payer],
                )
            )
            print(f"DApp initialized successfully. Transaction Signature: {tx}")
        except RPCException as e:
            print(f"Error initializing DApp: {e}")
            traceback.print_exc()
            raise
    else:
        print("DApp account already exists. Skipping initialization.")

    return dapp_pda

async def initialize_mint(program: Program, client: AsyncClient, dapp_pda: Pubkey) -> (Pubkey, Pubkey):
    """
    Initialize the Mint for the DApp, returning (mint_authority_pda, mint_for_dapp_pda).
    Sets dapp.mint_pubkey on-chain.
    """
    print("\nInitializing Mint for DApp...")

    # seeds for each PDA
    (mint_authority_pda, _) = Pubkey.find_program_address([b"mint_authority"], program.program_id)
    (mint_for_dapp_pda, _)  = Pubkey.find_program_address([b"my_spl_mint"], program.program_id)

    # Check if they might already exist
    mint_auth_acct = await client.get_account_info(mint_authority_pda, commitment=Confirmed)
    mint_acct      = await client.get_account_info(mint_for_dapp_pda, commitment=Confirmed)
    if mint_auth_acct.value is not None and mint_acct.value is not None:
        print("Mint + MintAuthority accounts already exist; skipping.")
        return (mint_authority_pda, mint_for_dapp_pda)

    try:
        token_pid = SPL_TOKEN_PROGRAM_ID

        tx = await program.rpc["initialize_mint"](
            ctx=Context(
                accounts={
                    "dapp":            dapp_pda,
                    "mint_authority":  mint_authority_pda,
                    "mint_for_dapp":   mint_for_dapp_pda,
                    "payer":           program.provider.wallet.public_key,
                    "token_program":   token_pid,
                    "system_program":  SYS_PROGRAM_ID,
                    "rent": Pubkey.from_string("SysvarRent111111111111111111111111111111111"),
                },
                signers=[program.provider.wallet.payer],
            )
        )
        print(f"InitializeMint => Transaction Signature: {tx}")
    except RPCException as e:
        print(f"Error initializing the mint: {e}")
        traceback.print_exc()
        raise

    return (mint_authority_pda, mint_for_dapp_pda)

async def initialize_game(
    program: Program, client: AsyncClient, game_number: int, description: str, dapp_pda: Pubkey
) -> Pubkey:
    """Initialize the Game, returning the Game PDA."""
    print("\nInitializing Game account...")
    game_pda, game_bump = Pubkey.find_program_address(
        [b"game", game_number.to_bytes(4, "little")], program.program_id
    )
    print(f"Game PDA: {game_pda}, Bump: {game_bump}")

    game_account = await client.get_account_info(game_pda, commitment=Confirmed)
    if game_account.value is None:
        try:
            tx = await program.rpc["initialize_game"](
                game_number,
                description,
                ctx=Context(
                    accounts={
                        "game": game_pda,
                        "dapp": dapp_pda,
                        "user": program.provider.wallet.public_key,
                        "system_program": SYS_PROGRAM_ID,
                    },
                    signers=[program.provider.wallet.payer],
                )
            )
            print(f"Game initialized successfully. Transaction Signature: {tx}")
        except RPCException as e:
            print(f"Error initializing Game: {e}")
            traceback.print_exc()
            raise
    else:
        print("Game account already exists. Skipping initialization.")

    return game_pda

async def register_validator_pda(
    program: Program,
    client: AsyncClient,
    game_pda: Pubkey,
    game_number: int,
    validator_kp: Keypair
) -> Pubkey:
    """Create a new validator PDA for the given user-based seeds."""
    print("\nRegistering a new Validator PDA (user-based seeds)...")
    game_number_bytes = game_number.to_bytes(4, "little")
    validator_key_bytes = bytes(validator_kp.pubkey())
    seeds = [b"validator", game_number_bytes, validator_key_bytes]
    (validator_pda, validator_pda_bump) = Pubkey.find_program_address(seeds, program.program_id)
    print(f"[DEBUG] Derived validator_pda = {validator_pda}, Bump = {validator_pda_bump}")

    try:
        tx_sig = await program.rpc["register_validator_pda"](
            game_number,
            ctx=Context(
                accounts={
                    "game": game_pda,
                    "validator_pda": validator_pda,
                    "user": validator_kp.pubkey(),
                    "system_program": SYS_PROGRAM_ID,
                },
                signers=[validator_kp],
            )
        )
        print(f"ValidatorPda registered. Tx Sig: {tx_sig}")
    except RPCException as e:
        print(f"Error registering Validator PDA: {e}")
        traceback.print_exc()
        raise

    return validator_pda

async def punch_in(program: Program, game_pda: Pubkey, game_number: int, validator_kp: Keypair):
    """Punch in as validator for the given game."""
    print("\nPunching In as Validator...")
    try:
        tx = await program.rpc["punch_in"](
            game_number,
            ctx=Context(
                accounts={
                    "game": game_pda,
                    "validator": validator_kp.pubkey(),
                    "system_program": SYS_PROGRAM_ID,
                },
                signers=[validator_kp],
            )
        )
        print(f"Punched in successfully. Transaction Signature: {tx}")
    except RPCException as e:
        print(f"Error punching in: {e}")
        traceback.print_exc()
        raise

async def register_player_pda(
    program: Program,
    client: AsyncClient,
    dapp_pda: Pubkey,
    name: str,
    authority_kp: Keypair,
    reward_address: Pubkey,
):
    """Register a PlayerPda using the dapp.global_player_count approach."""
    print("\nRegistering a new Player PDA...")

    # 1) Fetch the DApp to get global_player_count
    dapp_data = await program.account["DApp"].fetch(dapp_pda)
    current_count = dapp_data.global_player_count
    print(f"[DEBUG] dapp.global_player_count = {current_count}")

    # 2) seeds = [b"player_pda", current_count.to_bytes(4, "little")]
    count_bytes = current_count.to_bytes(4, "little")
    (player_pda, p_bump) = Pubkey.find_program_address([b"player_pda", count_bytes], program.program_id)
    print(f"[DEBUG] Derived player_pda = {player_pda}, Bump = {p_bump}")

    try:
        tx_sig = await program.rpc["register_player_pda"](
            name,
            authority_kp.pubkey(),
            reward_address,
            ctx=Context(
                accounts={
                    "dapp": dapp_pda,
                    "player_pda": player_pda,
                    "user": program.provider.wallet.public_key,
                    "system_program": SYS_PROGRAM_ID,
                },
                signers=[program.provider.wallet.payer],
            )
        )
        print(f"PlayerPda '{name}' created => {player_pda}. Tx Sig: {tx_sig}")
    except RPCException as e:
        print(f"Error registering PlayerPda '{name}': {e}")
        traceback.print_exc()
        raise


async def submit_minting_list(
    program: Program,
    dapp_pda: Pubkey,
    game_pda: Pubkey,
    game_number: int,
    validator_kp: Keypair,
    validator_pda: Pubkey,
    fancy_mint: Pubkey,
    mint_auth_pda: Pubkey,
    leftover_player_pda: Pubkey,
    leftover_player_ata: Pubkey,
):
    """
    Example usage: single leftover (PlayerPda, ATA) => one "player_id".
    The on-chain code expects leftover=2 per player => [PlayerPda, ATA].
    
    NOTE: Mark the PlayerPda leftover as `is_writable=True` so we can update it on-chain.
    """
    print("\nSubmitting Minting List => leftover [PlayerPda, ATA]...")

    # leftover => 2 per player
    leftover_accounts = [
        # IMPORTANT: Mark the PlayerPda as is_writable=True
        AccountMeta(pubkey=leftover_player_pda, is_signer=False, is_writable=True),
        # The ATA also needs to be writable if we do a `mint_to` on it
        AccountMeta(pubkey=leftover_player_ata, is_signer=False, is_writable=True),
    ]

    # We'll pass a single "player_id"
    player_ids = [999]  # example

    try:
        token_pid = SPL_TOKEN_PROGRAM_ID

        tx_sig = await program.rpc["submit_minting_list"](
            game_number,
            player_ids,
            ctx=Context(
                accounts={
                    "game":           game_pda,
                    "validator_pda":  validator_pda,
                    "validator":      validator_kp.pubkey(),
                    "fancy_mint":     fancy_mint,
                    "dapp":           dapp_pda,
                    "mint_authority": mint_auth_pda,
                    "token_program":  token_pid,
                },
                signers=[validator_kp],
                remaining_accounts=leftover_accounts,
            ),
        )
        print(f"Minting list submitted. Tx: {tx_sig}")
    except RPCException as e:
        print(f"Error in submit_minting_list: {e}")
        traceback.print_exc()

async def main():
    try:
        print("Setting up provider and loading program...")
        client = AsyncClient("http://localhost:8899", commitment=Confirmed)
        wallet = Wallet.local()
        provider = Provider(client, wallet)

        # Load IDL
        idl_path = Path("../target/idl/fancoin.json")
        if not idl_path.exists():
            print(f"IDL file not found at {idl_path.resolve()}")
            return

        with idl_path.open() as f:
            idl_json = f.read()

        program_id = Pubkey.from_string("HP9ucKGU9Sad7EaWjrGULC2ZSyYD1ScxVPh15QmdRmut")
        idl = Idl.from_json(idl_json)
        program = Program(idl, program_id, provider)
        print("Program loaded successfully.")

        # 1) Initialize the Dapp
        dapp_pda = await initialize_dapp(program, client)

        # 2) Initialize the Mint
        (mint_auth_pda, mint_for_dapp_pda) = await initialize_mint(program, client, dapp_pda)

        # 3) Initialize a Game
        game_number = 1
        description = "Minimal Game Example"
        game_pda = await initialize_game(program, client, game_number, description, dapp_pda)

        # 4) Register a validator
        def load_keypair(path: str) -> Keypair:
            with Path(path).open() as f:
                secret = json.load(f)
            return Keypair.from_bytes(bytes(secret[0:64]))

        validator_kp = load_keypair("./val1-keypair.json")
        validator_pda = await register_validator_pda(program, client, game_pda, game_number, validator_kp)

        # 5) Punch in as validator
        await punch_in(program, game_pda, game_number, validator_kp)

        # 6) Register a new Player
        player_kp = load_keypair("./player-keypair.json")
        reward_address = player_kp.pubkey()
        await register_player_pda(
            program, client, dapp_pda,
            name="Alice",
            authority_kp=player_kp,
            reward_address=reward_address
        )

        # 7) Suppose we create a dummy ATA for the "Alice" player:
        dummy_player_ata = Pubkey.find_program_address([b"dummyATA"], program_id)[0]

        # For simplicity, let's guess the new PlayerPda is index 0
        (alice_pda, _) = Pubkey.find_program_address([b"player_pda", (0).to_bytes(4, "little")], program_id)

        # 8) Now submit a single player's leftover => [alice_pda, dummy_player_ata]
        #   We must pass dapp_pda as well
        await submit_minting_list(
            program,
            dapp_pda=dapp_pda,          # <--- pass in the known DApp
            game_pda=game_pda,
            game_number=game_number,
            validator_kp=validator_kp,
            validator_pda=validator_pda,
            fancy_mint=mint_for_dapp_pda,
            mint_auth_pda=mint_auth_pda,
            leftover_player_pda=alice_pda,
            leftover_player_ata=dummy_player_ata,
        )

        print("\nAll tests completed successfully.")

    except Exception as e:
        print(f"An unexpected error occurred.\n{e}")
        traceback.print_exc()
    finally:
        await client.close()
        print("Closed Solana RPC client.")


if __name__ == "__main__":
    asyncio.run(main())
