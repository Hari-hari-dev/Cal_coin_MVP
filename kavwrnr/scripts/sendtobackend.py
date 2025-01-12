import asyncio
import json
from pathlib import Path

from anchorpy import Provider, Wallet
from solana.rpc.async_api import AsyncClient
from solana.rpc.commitment import Confirmed
from solana.rpc.core import RPCException
from solders.pubkey import Pubkey
from solders.system_program import transfer, TransferParams
from solana.transaction import Transaction

async def main():
    # 1. Connect to Devnet
    client = AsyncClient("https://api.devnet.solana.com", commitment=Confirmed)
    
    # 2. Use AnchorPy's local wallet (by default, reads ~/.config/solana/id.json)
    wallet = Wallet.local()
    
    # 3. Create a Provider using the client + wallet
    provider = Provider(client, wallet)

    # 4. Set the recipient's public key (the one you provided)
    recipient_pubkey = Pubkey.from_string("6awvSWyeJ4Y5u3ZX7soDJ23jQVkg25qLHxqeQssmELMJ")

    # 5. We want to send 1 SOL = 1_000_000_000 lamports
    lamports_to_send = 1_000_000_000

    # 6. Create the transfer instruction
    ix = transfer(
        TransferParams(
            from_pubkey=wallet.public_key,     # Sender (local wallet)
            to_pubkey=recipient_pubkey,         # Recipient
            lamports=lamports_to_send
        )
    )

    # 7. Build a transaction
    tx = Transaction()
    tx.add(ix)

    # 8. Fetch a recent blockhash
    latest_blockhash_resp = await provider.connection.get_latest_blockhash()
    if latest_blockhash_resp.value is None:
        raise Exception("Failed to get a recent blockhash")
    
    blockhash = latest_blockhash_resp.value.blockhash

    # 9. Set transaction blockhash & fee payer
    tx.recent_blockhash = blockhash
    tx.fee_payer = wallet.public_key

    try:
        # 10. Sign the transaction with the provider's wallet
        signed_tx = provider.wallet.sign_transaction(tx)

        # 11. Send the transaction
        resp = await provider.connection.send_raw_transaction(signed_tx.serialize())
        print(f"Transaction Signature: {resp}")
        print("Transfer completed successfully.")
    except RPCException as e:
        print(f"Error transferring lamports: {e}")

    # 12. Close the client connection
    await client.close()

if __name__ == "__main__":
    asyncio.run(main())
