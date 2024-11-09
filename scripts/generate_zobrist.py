import random

def get_hashes():
    values = [0 for _ in range(35*45)]
    for i in range(35):
        for j in range(45):
            values[i * 45 + j] = random.randint(0, 0xffffffffffffffff)
    return values

values = get_hashes()
while len(set(values)) != 1575:
    values = get_hashes()

print("pub const ZOBRIST_TABLE: [usize; 1575] = [")
for value in values:
    print(f"    0x{value:016X},")
print("];")
