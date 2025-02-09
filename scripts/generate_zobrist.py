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
print("//! This module implements the lookup tables used to hash a position.")
print()
print("/// Associates a piece's type index and cell index to its score, `index = piece_index*45 + cell_index`")
print("pub const ZOBRIST_TABLE: [usize; 1575] = [")
for value in values:
    print(f"    0x{value:016X},")
print("];")
