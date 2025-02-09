import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from itertools import product
from collections import defaultdict
from math import gcd


def part1(data: List[str]) -> Any:
    """ 2024 Day 8 Part 1
    >>> part1(['............', '........0...', '.....0......', '.......0....', '....0.......', '......A.....', '............', '............', '........A...', '.........A..', '............', '............'])
    14
    """

    antennas = defaultdict(set)
    for y, line in enumerate(data):
        for x, c in enumerate(line):
            if c != '.':
                antennas[c].add((x, y))

    bounds = (len(data[0]), len(data))
    anti_nodes = set()
    for ant_locs in antennas.values():
        for p1, p2 in product(ant_locs, repeat=2):
            offset = (p2[0] - p1[0], p2[1] - p1[1])
            if offset == (0, 0):
                continue

            point = tuple(p - o for p, o in zip(p1, offset))
            if all(0 <= p < b for p, b in zip(point, bounds)):
                anti_nodes.add(point)

            point = tuple(p + o for p, o in zip(p2, offset))
            if all(0 <= p < b for p, b in zip(point, bounds)):
                anti_nodes.add(point)

    return len(anti_nodes)


def part2(data: List[str]) -> Any:
    """ 2024 Day 8 Part 2
    >>> part2(['............', '........0...', '.....0......', '.......0....', '....0.......', '......A.....', '............', '............', '........A...', '.........A..', '............', '............'])
    34
    """

    antennas = defaultdict(set)
    for y, line in enumerate(data):
        for x, c in enumerate(line):
            if c != '.':
                antennas[c].add((x, y))

    bounds = (len(data[0]), len(data))
    anti_nodes = set()
    for ant_locs in antennas.values():
        for p1, p2 in product(ant_locs, repeat=2):
            offset = (p2[0] - p1[0], p2[1] - p1[1])
            if offset == (0, 0):
                continue

            # Not *technically* required, but good to check
            g = gcd(*offset)
            offset = tuple(o // g for o in offset)

            point = p1
            while all(0 <= p < b for p, b in zip(point, bounds)):
                anti_nodes.add(point)
                point = tuple(p - o for p, o in zip(point, offset))

            point = p2
            while all(0 <= p < b for p, b in zip(point, bounds)):
                anti_nodes.add(point)
                point = tuple(p + o for p, o in zip(point, offset))

    return len(anti_nodes)


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
        print(f"\nPart 1:\nNumber of antinodes: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nNumber of antinodes: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return (p1, p1_time.elapsed), (p2, p2_time.elapsed)


if __name__ == "__main__":
    main(verbose=True)