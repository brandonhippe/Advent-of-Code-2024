import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import numpy as np


def check_valid(diffs: List[int]) -> bool:
    """ Check if a list of differences is valid
    """
    return all(1 <= d <= 3 for d in diffs) or all(-3 <= d <= -1 for d in diffs)


def part1(data: List[str]) -> Any:
    """ 2024 Day 2 Part 1
    >>> part1(["7 6 4 2 1", "1 2 7 8 9", "9 7 6 2 1", "1 3 2 4 5", "8 6 4 4 1", "1 3 6 7 9"])
    2
    """

    return sum(check_valid(np.diff([int(x) for x in line.split()])) for line in data)


def part2(data: List[str]) -> Any:
    """ 2024 Day 2 Part 2
    >>> part2(["7 6 4 2 1", "1 2 7 8 9", "9 7 6 2 1", "1 3 2 4 5", "8 6 4 4 1", "1 3 6 7 9"])
    4
    """
    count = 0
    for line in data:
        base_nums = [int(x) for x in line.split()]
        diffs = np.diff(base_nums)
        if check_valid(diffs):
            count += 1
            continue
        
        for ix in range(len(base_nums)):
            nums = base_nums[:ix] + base_nums[ix+1:]
            
            diffs = np.diff(nums)
            if check_valid(diffs):
                count += 1
                break

    return count


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
        print(f"\nPart 1:\nValid count: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nValid count: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return (p1, p1_time.elapsed), (p2, p2_time.elapsed)


if __name__ == "__main__":
    main(verbose=True)