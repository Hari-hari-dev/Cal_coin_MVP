import json
import hashlib

IDL_PATH = "../target/idl/daily_facescan.json"  # Adjust if needed

def main():
    with open(IDL_PATH, "r") as f:
        idl = json.load(f)

    instructions = idl.get("instructions", [])
    print(f"Found {len(instructions)} instructions in IDL.")

    for instr in instructions:
        name = instr["name"]
        # e.g. "initialize", "claim", ...
        # 1) "global:NAME"
        discriminator_string = f"global:{name}"

        # 2) sha256, then take first 8 bytes => 16 hex chars
        full_hash = hashlib.sha256(discriminator_string.encode("utf-8")).hexdigest()
        disc_hex_16 = full_hash[:16]  # first 8 bytes => 16 hex chars

        print(f"Instruction '{name}': 0x{disc_hex_16}")

if __name__ == "__main__":
    main()
