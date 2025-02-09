import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import numpy as np
from itertools import product


def part1(data: List[str]) -> Any:
    """ 2024 Day 25 Part 1
    >>> part1(["#####", ".####", ".####", ".####", ".#.#.", ".#...", ".....", "", "#####", "##.##", ".#.##", "...##", "...#.", "...#.", ".....", "", ".....", "#....", "#....", "#...#", "#.#.#", "#.###", "#####", "", ".....", ".....", "#.#..", "###..", "###.#", "###.#", "#####", "", ".....", ".....", ".....", "#....", "#.#..", "#.#.#", "#####"])
    3
    """
    if len(data[-1]) != 0:
        data.append('')

    locks, keys = [], []
    curr = None
    max_height = 0
    for line in data:
        if len(line) == 0:
            max_height = max(max_height, max(curr[-1]))
            curr = None
            continue

        if curr is None:
            curr = locks if set(line).issubset(set('#')) else keys
            curr.append(np.zeros(len(line), dtype=int))

        for x, c in enumerate(line):
            curr[-1][x] += 1 if c == '#' else 0

    total = 0
    for l, k in product(locks, keys):
        comb = l + k
        if (comb <= max_height + 1).all():
            total += 1

    return total


def part2(data: List[str]) -> Any:
    """ 2024 Day 25 Part 2
    """

    return "Christmas has been saved!"


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
        print(f"\nPart 1:\nNumber of key/lock pairs that fit: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\n{p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return (p1, p1_time.elapsed), (p2, p2_time.elapsed)


if __name__ == "__main__":
    main(verbose=True)