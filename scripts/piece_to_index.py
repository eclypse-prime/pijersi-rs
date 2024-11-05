"""Generates the code that uses a lookup table to convert a piece's u8 representation to an index so it can be used by other lookup tables."""

conversion = {'S':1, 'P': 5, 'R': 9, 'W': 13, 's': 3, 'p': 7, 'r': 11, 'w': 15}

def piece_to_int(piece: str):
    if len(piece) == 1:
        return conversion[piece]
    else:
        return conversion[piece[0]] + conversion[piece[1]]*16

pieces = []

for bottom in ['S', 'P', 'R']:
    for top in ['S', 'P', 'R']:
        pieces.append(top + bottom)

for top in ['S', 'P', 'R', 'W']:
    pieces.append(top + 'W')

pieces += ['S', 'P', 'R', 'W']

for bottom in ['s', 'p', 'r']:
    for top in ['s', 'p', 'r']:
        pieces.append(top + bottom)

for top in ['s', 'p', 'r', 'w']:
    pieces.append(top + 'w')

pieces += ['s', 'p', 'r', 'w']


indices = {}

n = 0
for piece in pieces:
    indices[piece_to_int(piece)] = n
    n += 1

print("pub const PIECE_TO_INDEX: [usize; 256] = [")
for i in range(256):
    if i in indices:
        print(f"    {indices[i]}", end='')
    else:
        print("    34", end='')
    if i != 255:
        print(",")
    else:
        print()
print("];")