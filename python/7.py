import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from multiprocessing import Pool

def test_operators(nums: List[int], goal: int) -> int:
    for i in range(3 ** (len(nums) - 1)):
        test = nums[0]
        for j in range(len(nums) - 1):
            if (i // (3 ** j)) % 3 == 0:
                test += nums[j + 1]
            elif (i // (3 ** j)) % 3 == 1:
                test *= nums[j + 1]
            else:
                test = int(str(test) + str(nums[j + 1]))

        if test == goal:
            return goal

    return 0


def part1(data: List[str]) -> Any:
    """ 2024 Day 7 Part 1
    >>> part1(["190: 10 19", "3267: 81 40 27", "83: 17 5", "156: 15 6", "7290: 6 8 6 15", "161011: 16 10 13", "192: 17 8 14", "21037: 9 7 18 13", "292: 11 6 16 20"])
    3749
    """
    total = 0
    for line in data:
        goal, nums = line.split(': ')
        goal = int(goal)

        nums = list(map(int, nums.split()))
        for i in range(2 ** (len(nums) - 1)):
            test = nums[0]
            for j in range(len(nums) - 1):
                if (i >> j) & 1:
                    test += nums[j + 1]
                else:
                    test *= nums[j + 1]

            if test == goal:
                total += goal
                break

    return total


def part2(data: List[str]) -> Any:
    """ 2024 Day 7 Part 2
    >>> part2(["190: 10 19", "3267: 81 40 27", "83: 17 5", "156: 15 6", "7290: 6 8 6 15", "161011: 16 10 13", "192: 17 8 14", "21037: 9 7 18 13", "292: 11 6 16 20"])
    11387
    """
    return sum(Pool().starmap(test_operators, [(list(map(int, line.split(': ')[1].split())), int(line.split(': ')[0])) for line in data]))


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
        print(f"\nPart 1:\nSum of valid combinations: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nSum of valid combinations: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return (p1, p1_time.elapsed), (p2, p2_time.elapsed)


if __name__ == "__main__":
    main(verbose=True)