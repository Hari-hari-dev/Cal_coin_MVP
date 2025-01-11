import json
import base64

# Path to your Solana keypair JSON file
KEYPAIR_JSON = "devmqauyKoRLubFAy3Af5eWspU6BmNRnM5YTuQGSJJ2.json"

def main():
    # 1) Load the JSON file
    with open(KEYPAIR_JSON, "r") as f:
        key_data = json.load(f)  # Typically an array of 64 integers

    # 2) Convert that array of integers to a bytes object (64 bytes)
    secret_bytes = bytes(key_data)

    # 3) Confirm the length is 64
    if len(secret_bytes) != 64:
        print(f"Warning: expected 64 bytes, but found {len(secret_bytes)}.")
    else:
        print("Keypair length is 64 bytes.")

    # 4a) Print as base64
    base64_str = base64.b64encode(secret_bytes).decode("utf-8")
    print("\nBase64 (64-byte Ed25519 keypair):")
    print(base64_str)

    # 4b) Print as hex (optional)
    hex_str = secret_bytes.hex()
    print("\nHex (64-byte Ed25519 keypair):")
    print(hex_str)

if __name__ == "__main__":
    main()
