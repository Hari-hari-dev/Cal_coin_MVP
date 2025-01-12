import base64

# 32-byte seed array
seed_32 = [
    172, 12, 62, 189, 37, 88, 142, 186,
    14, 212, 222, 9, 162, 43, 133, 112,
    59, 255, 236, 7, 249, 177, 70, 221,
    56, 87, 93, 174, 157, 210, 44, 200
]

# Convert list of ints to bytes
seed_bytes = bytes(seed_32)

# Encode as base64 (returns a bytes object), then decode to string
seed_base64 = base64.b64encode(seed_bytes).decode("utf-8")

print("Base64 Encoded Seed:", seed_base64)
