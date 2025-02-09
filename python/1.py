import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from collections import defaultdict


def split_lists(data: List[str]) -> Tuple[List[int], List[int]]:
    lists = [[], []]
    for line in data:
        for ix, n in enumerate(re.findall(r"\d+", line)):
            assert 0 <= ix < 2, "Only two lists are allowed"
            lists[ix].append(int(n))

    return *lists,


def part1(data: List[str]) -> Any:
    """ 2024 Day 1 Part 1
    >>> part1(["3   4", "4   3", "2   5", "1   3", "3   9", "3   3"])
    11
    """
    left_list, right_list = split_lists(data)

    left_list.sort()
    right_list.sort()

    return sum(abs(l - r) for l, r in zip(left_list, right_list))


def part2(data: List[str]) -> Any:
    """ 2024 Day 1 Part 2
    >>> part2(["3   4", "4   3", "2   5", "1   3", "3   9", "3   3"])
    31
    """
    left_list, right_list = split_lists(data)

    right_counts = defaultdict(int)
    for r in right_list:
        right_counts[r] += 1

    return sum(l * right_counts[l] for l in left_list)


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
        print(f"\nPart 1:\nTotal distance: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nSimilarity Score: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return (p1, p1_time.elapsed), (p2, p2_time.elapsed)


if __name__ == "__main__":
    main(verbose=True)