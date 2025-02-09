import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from collections import defaultdict


def iterate_stones(stones: dict, amt: int) -> dict:
    for _ in range(amt):
        new_stones = defaultdict(int)
        for stone, count in stones.items():
            if stone == 0:
                new_stones[1] += count
                continue

            s = str(stone)
            if len(s) % 2 == 0:
                new_stones[int(s[:len(s) // 2])] += count
                new_stones[int(s[len(s) // 2:])] += count
            else:
                new_stones[stone * 2024] += count

        stones = new_stones

    return stones


def part1(data: List[str], amt: int=25) -> Any:
    """ 2024 Day 11 Part 1
    >>> part1(["125 17"], 6)
    22
    >>> part1(["125 17"], 25)
    55312
    """

    stones = defaultdict(int)

    for n in data[0].split():
        stones[int(n)] += 1

    return sum(iterate_stones(stones, amt).values())


def part2(data: List[str]) -> Any:
    """ 2024 Day 11 Part 2
    """

    stones = defaultdict(int)

    for n in data[0].split():
        stones[int(n)] += 1

    return sum(iterate_stones(stones, 75).values())


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
        print(f"\nPart 1:\nStones after 25 iterations: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nStones after 75 iterations: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return (p1, p1_time.elapsed), (p2, p2_time.elapsed)


if __name__ == "__main__":
    main(verbose=True)