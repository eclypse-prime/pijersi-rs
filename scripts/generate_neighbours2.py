"""A helper script to generate the neighbouring cells table `NEIGHBOURS1: [Bitboard; 45]`
I wanted to hardcode this so I created this script to generate all the cases in Rust code.
"""

from typing import List


def coords_index(i: int, j: int) -> int:
    if i % 2 == 0:
        index = 13 * i // 2 + j
    else:
        index = 6 + 13 * (i-1) // 2 + j
    return index


def find_neighbours(i: int, j: int) -> List[int]:
    neighbours = []
    index = coords_index(i, j)
    if j > 0:
        if j > 1:
            neighbours.append(index - 2)
        if i > 1:
            neighbours.append(index - 14)
        if i < 5:
            neighbours.append(index + 12)

    if (i % 2 == 0 and j < 5) or (i % 2 == 1 and j < 6):
        if (i % 2 == 0 and j < 4) or (i % 2 == 1 and j < 5):
            neighbours.append(index + 2)
        if i > 1:
            neighbours.append(index - 12)
        if i < 5:
            neighbours.append(index + 14)

    neighbours.sort()
    return neighbours


def print_case(i: int, j: int):
    neighbours = find_neighbours(i, j)
    mask = 0
    for neighbour in neighbours:
        mask += 1 << neighbour
    print(f"    Bitboard({mask}),")

print("pub const NEIGHBOURS2: [Bitboard; 45] = [")
for i in range(7):
    if i % 2 == 0:
        for j in range(6):
            print_case(i, j)
    else:
        for j in range(7):
            print_case(i, j)
print("];")