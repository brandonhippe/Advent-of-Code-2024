import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from collections import defaultdict
import heapq


def part1(data: List[str]) -> Any:
    """ 2024 Day 9 Part 1
    >>> part1(["12345"])
    60
    >>> part1(["2333133121414131402"])
    1928
    """

    data_blocks = []
    start_ix = 0
    add_block = True
    for amt in map(int, data[0]):
        if add_block:
            data_blocks.append((start_ix, start_ix + amt))

        add_block = not add_block
        start_ix += amt

    left_ix, right_ix = 0, len(data_blocks) - 1
    left_pos, right_pos = 0, data_blocks[-1][-1] - 1

    checksum = 0

    while left_pos <= right_pos:
        if right_pos < data_blocks[right_ix][0]:
            right_ix -= 1
            right_pos = data_blocks[right_ix][1] - 1

        if left_pos == data_blocks[left_ix][1]:
            left_ix += 1

        if left_pos > right_pos:
            break

        if left_pos < data_blocks[left_ix][0]:
            checksum += left_pos * right_ix
            right_pos -= 1
        elif left_pos < data_blocks[left_ix][1]:
            checksum += left_pos * left_ix

        left_pos += 1

    return checksum


def part2(data: List[str]) -> Any:
    """ 2024 Day 9 Part 2
    >>> part2(["2333133121414131402"])
    2858
    """

    data_blocks = {}
    free_spaces = defaultdict(list)
    start_ix = 0
    for ix, amt in enumerate(map(int, data[0])):
        if ix % 2 == 0:
            data_blocks[ix // 2] = (start_ix, start_ix + amt)
        elif amt > 0:
            heapq.heappush(free_spaces[amt], start_ix)

        start_ix += amt

    for ix in list(data_blocks.keys())[::-1]:
        start, end = data_blocks[ix]
        amt = end - start

        earliest = start
        space_ix = None
        for size, spaces in free_spaces.items():
            if size >= amt:
                if spaces[0] < earliest:
                    earliest = spaces[0]
                    space_ix = size

        if space_ix:
            heapq.heappop(free_spaces[space_ix])
            if len(free_spaces[space_ix]) == 0:
                del free_spaces[space_ix]

            data_blocks[ix] = (earliest, earliest + amt)
            if space_ix > amt:
                heapq.heappush(free_spaces[space_ix - amt], earliest + amt)

    return sum(sum(block_id * ix for ix in range(*r)) for block_id, r in data_blocks.items())


def main(input_path: Optional[Path | str]=None, verbose: bool=False) -> Tuple[Tuple[Any, float]]:
    if not input_path:
        if not (input_path := sys.argv[1] if len(sys.argv) > 1 else None):
            year, day = re.findall(r'\d+', str(__file__))[-2:]
            input_path = Path(Path(__file__).parent.parent.parent, "Inputs", f"{year}_{day}.txt")
    
    with open(input_path, encoding='UTF-8') as f:
        data = [line.strip('\n') for line in f.readlines()]

    with Timer() as p1_time:
        p1 = part1(data)

    if verbose:
        print(f"\nPart 1:\nFilesystem Checksum: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nDefragmented Filesystem Checksum: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return (p1, p1_time.elapsed), (p2, p2_time.elapsed)


if __name__ == "__main__":
    main(verbose=True)