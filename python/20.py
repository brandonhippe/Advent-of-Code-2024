import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from collections import defaultdict


def part1(data: List[str], test=False) -> Any:
    """ 2024 Day 20 Part 1
    >>> part1(["###############", "#...#...#.....#", "#.#.#.#.#.###.#", "#S#...#.#.#...#", "#######.#.#.###", "#######.#.#...#", "#######.#.###.#", "###..E#...#...#", "###.#######.###", "#...###...#...#", "#.#####.#.###.#", "#.#...#.#.#...#", "#.#.#.#.#.#.###", "#...#...#...###", "###############"], True)
    {2: 14, 4: 14, 6: 2, 8: 4, 10: 2, 12: 3, 20: 1, 36: 1, 38: 1, 40: 1, 64: 1}
    """
    area = set()
    start, end = None, None
    for y, line in enumerate(data):
        for x, c in enumerate(line):
            if c == '#':
                continue
            elif c == 'S':
                start = (x, y)
            elif c == 'E':
                end = (x, y)

            area.add((x, y))

    assert start is not None, "Start not found"
    assert end is not None, "End not found"

    curr_pos = start
    visited = set()
    path_order = []
    path_ixs = {}

    while True:
        path_ixs[curr_pos] = len(path_order)
        path_order.append(curr_pos)
        visited.add(curr_pos)
        if curr_pos == end:
            break

        for dx, dy in [(0, 1), (0, -1), (1, 0), (-1, 0)]:
            new_pos = (curr_pos[0] + dx, curr_pos[1] + dy)
            if new_pos in visited or new_pos not in area:
                continue

            next_pos = new_pos
            break

        curr_pos = next_pos

    path_reductions = defaultdict(int)
    for i, start_pos in enumerate(path_order):
        for dx, dy in [(0, 1), (0, -1), (1, 0), (-1, 0)]:
            wall_pos = (start_pos[0] + dx, start_pos[1] + dy)
            if wall_pos in area:
                continue

            next_pos = (wall_pos[0] + dx, wall_pos[1] + dy)
            if next_pos in area and path_ixs[next_pos] > i:
                path_reductions[path_ixs[next_pos] - i - 2] += 1

    if test:
        return {k: path_reductions[k] for k in sorted(path_reductions.keys())}

    return sum(v for k, v in path_reductions.items() if k >= 100)


def part2(data: List[str], test=False) -> Any:
    """ 2024 Day 20 Part 2
    >>> part2(["###############", "#...#...#.....#", "#.#.#.#.#.###.#", "#S#...#.#.#...#", "#######.#.#.###", "#######.#.#...#", "#######.#.###.#", "###..E#...#...#", "###.#######.###", "#...###...#...#", "#.#####.#.###.#", "#.#...#.#.#...#", "#.#.#.#.#.#.###", "#...#...#...###", "###############"], True)
    285
    """
    area = set()
    walls = set()
    start, end = None, None
    for y, line in enumerate(data):
        for x, c in enumerate(line):
            if c == '#':
                walls.add((x, y))
                continue
            elif c == 'S':
                start = (x, y)
            elif c == 'E':
                end = (x, y)

            area.add((x, y))

    assert start is not None, "Start not found"
    assert end is not None, "End not found"

    curr_pos = start
    visited = set()
    path_order = []
    path_ixs = {}

    while True:
        path_ixs[curr_pos] = len(path_order)
        path_order.append(curr_pos)
        visited.add(curr_pos)
        if curr_pos == end:
            break

        for dx, dy in [(0, 1), (0, -1), (1, 0), (-1, 0)]:
            new_pos = (curr_pos[0] + dx, curr_pos[1] + dy)
            if new_pos in visited or new_pos not in area:
                continue

            next_pos = new_pos
            break

        curr_pos = next_pos

    path_reductions = defaultdict(int)
    for i, start_pos in enumerate(path_order):
        for dx in range(-20, 21):
            y_lim = 20 - abs(dx)
            for dy in range(-y_lim, y_lim + 1):
                end_pos = (start_pos[0] + dx, start_pos[1] + dy)
                if end_pos in walls or end_pos not in area or path_ixs[end_pos] <= i:
                    continue

                dist = abs(dx) + abs(dy)
                path_reductions[path_ixs[end_pos] - i - dist] += 1

    return sum(v for k, v in path_reductions.items() if k >= (100 if not test else 50))


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
        print(f"\nPart 1:\nNumber of cheats that save at least 100 ps: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nNumber of cheats that save at least 100 ps: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return (p1, p1_time.elapsed), (p2, p2_time.elapsed)


if __name__ == "__main__":
    main(verbose=True)