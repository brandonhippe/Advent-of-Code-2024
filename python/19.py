import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from functools import cache
from collections import defaultdict


@cache
def possible_pattern(rem_pattern: str, available_towels: Tuple[Tuple[str]]) -> int:
    if not rem_pattern:
        return 1

    count = 0
    for towel_list in available_towels:
        l = len(towel_list[0])
        if l <= len(rem_pattern) and rem_pattern[:l] in towel_list:
            count += possible_pattern(rem_pattern[l:], available_towels)

    return count


def part1(data: List[str]) -> Any:
    """ 2024 Day 19 Part 1
    >>> part1(["r, wr, b, g, bwu, rb, gb, br", "", "brwrr", "bggr", "gbbr", "rrbgbr", "ubwu", "bwurrg", "brgr", "bbrgwb"])
    6
    """
    available_towels = defaultdict(list)
    for towel in data[0].split(", "):
        available_towels[len(towel)].append(towel)

    available_towels = tuple(tuple(v) for v in available_towels.values())
    return sum(possible_pattern(pattern, available_towels) != 0 for pattern in data[2:])


def part2(data: List[str]) -> Any:
    """ 2024 Day 19 Part 2
    >>> part2(["r, wr, b, g, bwu, rb, gb, br", "", "brwrr", "bggr", "gbbr", "rrbgbr", "ubwu", "bwurrg", "brgr", "bbrgwb"])
    16
    """
    available_towels = defaultdict(list)
    for towel in data[0].split(", "):
        available_towels[len(towel)].append(towel)

    available_towels = tuple(tuple(v) for v in available_towels.values())
    return sum(possible_pattern(pattern, available_towels) for pattern in data[2:])


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
        print(f"\nPart 1:\nNumber of possible designs: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nNumber of ways to make all possible designs: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return (p1, p1_time.elapsed), (p2, p2_time.elapsed)


if __name__ == "__main__":
    main(verbose=True)