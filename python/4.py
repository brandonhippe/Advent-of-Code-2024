import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from collections import defaultdict
from itertools import product


def part1(data: List[str]) -> Any:
    """ 2024 Day 4 Part 1
    >>> part1(["MMMSXXMASM", "MSAMXMSMSA", "AMXSXMAAMM", "MSAMASMSMX", "XMASAMXAMM", "XXAMMXXAMA", "SMSMSASXSS", "SAXAMASAAA", "MAMMMXMMMM", "MXMXAXMASX"])
    18
    """

    search_word = 'XMAS'
    letter_sets = {letter: set() for letter in search_word}
    for i, line in enumerate(data):
        for j, letter in enumerate(line):
            if letter in search_word:
                letter_sets[letter].add((i, j))

    total_count = 0
    open_list = defaultdict(list)
    open_list.update({pos: [(0, i, j) for i, j in product(range(-1, 2), repeat=2)] for pos in letter_sets[search_word[0]]})
    while len(open_list):
        pos, dir_list = open_list.popitem()

        for letter_ix, i, j in dir_list:
            if letter_ix == len(search_word) - 1:
                total_count += 1
                continue

            if 0 <= pos[0] + i < len(data) and 0 <= pos[1] + j < len(data[0]):
                if (pos[0] + i, pos[1] + j) in letter_sets[search_word[letter_ix + 1]]:
                    open_list[(pos[0] + i, pos[1] + j)].append((letter_ix + 1, i, j))

    return total_count


def part2(data: List[str]) -> Any:
    """ 2024 Day 4 Part 2
    >>> part2(["MMMSXXMASM", "MSAMXMSMSA", "AMXSXMAAMM", "MSAMASMSMX", "XMASAMXAMM", "XXAMMXXAMA", "SMSMSASXSS", "SAXAMASAAA", "MAMMMXMMMM", "MXMXAXMASX"])
    9
    """

    search_word = 'MAS'
    letter_sets = {letter: set() for letter in search_word}
    for i, line in enumerate(data):
        for j, letter in enumerate(line):
            if letter in search_word:
                letter_sets[letter].add((i, j))

    centers = defaultdict(set) 
    open_list = defaultdict(list)
    open_list.update({pos: [(0, i, j) for i, j in product(range(-1, 2, 2), repeat=2)] for pos in letter_sets[search_word[0]]})
    while len(open_list):
        pos, dir_list = open_list.popitem()

        for letter_ix, i, j in dir_list:
            if letter_ix == len(search_word) - 1:
                centers[(pos[0] - i, pos[1] - j)].add((i, j))
                continue

            if 0 <= pos[0] + i < len(data) and 0 <= pos[1] + j < len(data[0]):
                if (pos[0] + i, pos[1] + j) in letter_sets[search_word[letter_ix + 1]]:
                    open_list[(pos[0] + i, pos[1] + j)].append((letter_ix + 1, i, j))

    total_count = 0
    for directions in centers.values():
        for dx, dy in directions.copy():
            if (dx, dy) not in directions:
                continue

            for rotated_dx, rotated_dy in [(-dy, dx), (dy, -dx)]:
                if (rotated_dx, rotated_dy) in directions:
                    directions.remove((rotated_dx, rotated_dy))
                    directions.remove((dx, dy))
                    total_count += 1
                    break
        
    return total_count


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
        print(f"\nPart 1:\nNumber of XMAS's found: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nNumber of X-MAS's found: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return (p1, p1_time.elapsed), (p2, p2_time.elapsed)


if __name__ == "__main__":
    main(verbose=True)