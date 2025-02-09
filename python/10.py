import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from collections import defaultdict, deque


def part1(data: List[str]) -> Any:
    """ 2024 Day 10 Part 1
    >>> part1(["89010123", "78121874", "87430965", "96549874", "45678903", "32019012", "01329801", "10456732"])
    36
    """

    heights = defaultdict(set)
    for y, line in enumerate(data):
        for x, c in enumerate(line):
            heights[int(c)].add((x, y))

    score_sum = 0
    for pos in heights[0]:
        open_list = deque([(*pos, 0)])
        visited = set()
        finishes = set()
        while open_list:
            x, y, height = open_list.popleft()
            if height == 9:
                finishes.add((x, y))
                continue

            if (x, y) in visited:
                continue

            visited.add((x, y))

            for dx, dy in [(0, 1), (1, 0), (0, -1), (-1, 0)]:
                new_x, new_y = x + dx, y + dy
                if (new_x, new_y) in heights[height + 1]:
                    open_list.append((new_x, new_y, height + 1))

        score_sum += len(finishes)

    return score_sum


def part2(data: List[str]) -> Any:
    """ 2024 Day 10 Part 2
    >>> part2(["89010123", "78121874", "87430965", "96549874", "45678903", "32019012", "01329801", "10456732"])
    81
    """

    heights = defaultdict(set)
    for y, line in enumerate(data):
        for x, c in enumerate(line):
            heights[int(c)].add((x, y))

    score_sum = 0
    for pos in heights[0]:
        open_list = deque([(*pos, 0)])
        visited = set()
        finishes = defaultdict(int)
        while open_list:
            x, y, height = open_list.popleft()
            if height == 9:
                finishes[(x, y)] += 1
                continue

            visited.add((x, y))

            for dx, dy in [(0, 1), (1, 0), (0, -1), (-1, 0)]:
                new_x, new_y = x + dx, y + dy
                if (new_x, new_y) in heights[height + 1]:
                    open_list.append((new_x, new_y, height + 1))

        score_sum += sum(finishes.values())

    return score_sum


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
        print(f"\nPart 1:\nSum of trailhead scores: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nSum of trailhead ratings: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return (p1, p1_time.elapsed), (p2, p2_time.elapsed)


if __name__ == "__main__":
    main(verbose=True)