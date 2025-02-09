import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re


def part1(data: List[str]) -> Any:
    """ 2024 Day 3 Part 1
    >>> part1(["xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"])
    161
    """
    count = 0
    for line in data:
        for match in re.findall(r'mul\((\d+),(\d+)\)', line):
            count += int(match[0]) * int(match[1])

    return count


def part2(data: List[str]) -> Any:
    """ 2024 Day 3 Part 2
    >>> part2(["xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"])
    48
    """
    count = 0
    enabled = True
    for line in data:
        for match in re.findall(r"(mul\((\d+),(\d+)\))|(do\(\))|(don't\(\))", line):
            if len(match[1]) and len(match[2]):
                count += int(match[1]) * int(match[2]) * enabled
            elif len(match[3]):
                enabled = True
            elif len(match[4]):
                enabled = False
            else:
                raise ValueError("Invalid match")

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
        print(f"\nPart 1:\nSum of multiplication instructions: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nSum of enabled multiplication instructions: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return (p1, p1_time.elapsed), (p2, p2_time.elapsed)


if __name__ == "__main__":
    main(verbose=True)