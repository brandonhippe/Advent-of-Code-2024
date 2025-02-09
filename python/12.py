import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from collections import defaultdict, deque


def part1(data: List[str]) -> Any:
    """ 2024 Day 12 Part 1
    >>> part1(["AAAA", "BBCD", "BBCC", "EEEC"])
    140
    >>> part1(["OOOOO", "OXOXO", "OOOOO", "OXOXO", "OOOOO"])
    772
    >>> part1(["RRRRIICCFF", "RRRRIICCCF", "VVRRRCCFFF", "VVRCCCJFFF", "VVVVCJJCFE", "VVIVCCJJEE", "VVIIICJJEE", "MIIIIIJJEE", "MIIISIJEEE", "MMMISSJEEE"])
    1930
    """

    plot_types = defaultdict(set)
    for y, line in enumerate(data):
        for x, char in enumerate(line):
            plot_types[char].add((x, y))

    cost = 0
    for t, points in plot_types.items():
        start_points = list(points)
        while start_points:
            area = 0
            perimeter = 0

            to_check = deque([start_points[-1]])
            while to_check:
                pos = to_check.popleft()
                if pos in start_points:
                    start_points.remove(pos)
                    area += 1
                    x, y = pos
                    for dx, dy in ((1, 0), (-1, 0), (0, 1), (0, -1)):
                        new_pos = (x + dx, y + dy)
                        if new_pos in plot_types[t]:
                            to_check.append(new_pos)
                        else:
                            perimeter += 1

            cost += area * perimeter

    return cost


def part2(data: List[str]) -> Any:
    """ 2024 Day 12 Part 2
    >>> part2(["AAAA", "BBCD", "BBCC", "EEEC"])
    80
    >>> part2(["OOOOO", "OXOXO", "OOOOO", "OXOXO", "OOOOO"])
    436
    >>> part2(["EEEEE", "EXXXX", "EEEEE", "EXXXX", "EEEEE"])
    236
    >>> part2(["AAAAAA", "AAABBA", "AAABBA", "ABBAAA", "ABBAAA", "AAAAAA"])
    368
    >>> part2(["RRRRIICCFF", "RRRRIICCCF", "VVRRRCCFFF", "VVRCCCJFFF", "VVVVCJJCFE", "VVIVCCJJEE", "VVIIICJJEE", "MIIIIIJJEE", "MIIISIJEEE", "MMMISSJEEE"])
    1206
    """

    plot_types = defaultdict(set)
    for y, line in enumerate(data):
        for x, char in enumerate(line):
            plot_types[char].add((x, y))

    cost = 0
    for t, points in plot_types.items():
        start_points = list(points)
        while start_points:
            area = 0
            perimeter = defaultdict(set)

            to_check = deque([start_points[-1]])
            while to_check:
                pos = to_check.popleft()
                if pos in start_points:
                    start_points.remove(pos)
                    area += 1
                    x, y = pos
                    for dx, dy in ((1, 0), (-1, 0), (0, 1), (0, -1)):
                        new_pos = (x + dx, y + dy)
                        if new_pos in plot_types[t]:
                            to_check.append(new_pos)
                        else:
                            perimeter[(dx, dy)].add(new_pos)

            sides = 0
            for d, ps in perimeter.items():
                sps = list(ps)
                while sps:
                    to_check = deque([sps[-1]])
                    while to_check:
                        pos = to_check.popleft()
                        if pos in sps:
                            sps.remove(pos)
                            x, y = pos
                            for dx, dy in ((d[1], d[0]), (d[1], -d[0]), (-d[1], d[0]), (-d[1], -d[0])):
                                new_pos = (x + dx, y + dy)
                                if new_pos in perimeter[d]:
                                    to_check.append(new_pos)

                    sides += 1

            cost += area * sides

    return cost


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
        print(f"\nPart 1:\nCost of fence: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nCost of fence: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return (p1, p1_time.elapsed), (p2, p2_time.elapsed)


if __name__ == "__main__":
    main(verbose=True)