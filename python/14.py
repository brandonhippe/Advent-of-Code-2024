import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re
import numpy as np


class Robot:
    def __init__(self, init_str: str):
        self.x, self.y = (int(n) for n in re.search(r"p=(-?\d+),(-?\d+)", init_str).groups())
        self.xv, self.yv = (int(n) for n in re.search(r"v=(-?\d+),(-?\d+)", init_str).groups())

    def __repr__(self):
        return f"({self.x}, {self.y}) -> ({self.xv}, {self.yv})"
    
    def __hash__(self):
        return hash((self.x, self.y, self.xv, self.yv))
    
    def move(self, width: int, height: int):
        self.x += self.xv
        self.y += self.yv

        self.x %= width
        self.y %= height


def printRobots(positions: set[tuple[int,]], width: int, height: int):
    for y in range(height):
        for x in range(width):
            print('█' if (x, y) in positions else ' ', end='')
        print()


def part1(data: List[str], width: int=101, height: int=103) -> Any:
    """ 2024 Day 14 Part 1
    >>> part1(["p=0,4 v=3,-3", "p=6,3 v=-1,-3", "p=10,3 v=-1,2", "p=2,0 v=2,-1", "p=0,0 v=1,3", "p=3,0 v=-2,-2", "p=7,6 v=-1,-3", "p=3,0 v=-1,-2", "p=9,3 v=2,3", "p=7,3 v=-1,2", "p=2,4 v=2,-3", "p=9,5 v=-3,-3"], 11, 7)
    12
    """

    robots = [Robot(line) for line in data]
    for _ in range(100):
        for robot in robots:
            robot.move(width, height)


    quadrant_counts = [[0, 0], [0, 0]]
    for robot in robots:
        if robot.y == (height // 2) or robot.x == (width // 2):
            continue

        quadrant_counts[robot.y > (height // 2)][robot.x > (width // 2)] += 1

    return quadrant_counts[0][0] * quadrant_counts[1][1] * quadrant_counts[0][1] * quadrant_counts[1][0]


def part2(data: List[str]) -> Any:
    """ 2024 Day 14 Part 2
    """

    width, height = 101, 103
    robots = tuple(Robot(line) for line in data)
    seconds = 0

    # Find the time at which the variance of the x and y coordinates is minimized
    min_x_var, min_y_var = [float('inf')] * 2
    min_x_seconds, min_y_seconds = [-1] * 2
    while seconds < max(width, height):
        x_var = np.var([robot.x for robot in robots])
        if x_var < min_x_var:
            min_x_var = x_var
            min_x_seconds = seconds

        y_var = np.var([robot.y for robot in robots])
        if y_var < min_y_var:
            min_y_var = y_var
            min_y_seconds = seconds

        for robot in robots:
            robot.move(width, height)

        seconds += 1

    # Calculate final time using Chinese Remainder Theorem
    n_s = [width, height]
    a_s = [min_x_seconds, min_y_seconds]

    y_s = [np.prod([n_s[i] for i in range(len(n_s)) if i != j]) for j in range(len(n_s))]
    z_s = [pow(int(y), m - 2, m) for y, m in zip(y_s, n_s)]

    return sum(a * y * z for a, y, z in zip(a_s, y_s, z_s)) % np.prod(n_s)


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
        print(f"\nPart 1:\nSaftey Factor: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nTime for Christmas Tree to appear: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return (p1, p1_time.elapsed), (p2, p2_time.elapsed)


if __name__ == "__main__":
    main(verbose=True)