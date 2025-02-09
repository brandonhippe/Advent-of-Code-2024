import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import heapq
from collections import defaultdict


def manhattan_distance(p1, p2):
    return sum(abs(a - b) for a, b in zip(p1, p2))


def print_path(walls, path):
    (min_x, min_y), (max_x, max_y) = (min(p[i] for p in walls) for i in range(2)), (max(p[i] for p in walls) for i in range(2))

    for y in range(min_y, max_y + 1):
        for x in range(min_x, max_x + 1):
            if (x, y) in walls:
                print("█", end="")
            elif (x, y) in path:
                print("O", end="")
            else:
                print(" ", end="")
        print()


def next_states(pos, d, walls, cost):
    states = []
    for move in range(-1, 2):
        if move == 0:
            next_pos = tuple(p + o for p, o in zip(pos, d))
            next_d = d
            new_cost = cost + 1
        else:
            next_pos = pos[:]
            next_d = (move * -d[1], move * d[0])
            new_cost = cost + 1000

        next_state = (next_pos, next_d)

        if next_pos not in walls:
            states.append((next_state, new_cost))

    return states


def part1(data: List[str]) -> Any:
    """ 2024 Day 16 Part 1
    >>> part1(["###############", "#.......#....E#", "#.#.###.#.###.#", "#.....#.#...#.#", "#.###.#####.#.#", "#.#.#.......#.#", "#.#.#####.###.#", "#...........#.#", "###.#.#####.#.#", "#...#.....#.#.#", "#.#.#.###.#.#.#", "#.....#...#.#.#", "#.###.#.#.#.#.#", "#S..#.....#...#", "###############"])
    7036
    >>> part1(["#################", "#...#...#...#..E#", "#.#.#.#.#.#.#.#.#", "#.#.#.#...#...#.#", "#.#.#.#.###.#.#.#", "#...#.#.#.....#.#", "#.#.#.#.#.#####.#", "#.#...#.#.#.....#", "#.#.#####.#.###.#", "#.#.#.......#...#", "#.#.###.#####.###", "#.#.#...#.....#.#", "#.#.#.#####.###.#", "#.#.#.........#.#", "#.#.#.#########.#", "#S#.............#", "#################"])
    11048
    """
    walls = set()
    start = None
    end = None
    for y, line in enumerate(data):
        for x, c in enumerate(line):
            if c == "S":
                start = (x, y)
            elif c == "E":
                end = (x, y)
            elif c == "#":
                walls.add((x, y))

    assert start is not None, "Start not found"
    assert end is not None, "End not found"

    open_dict = {(start, (1, 0)): manhattan_distance(start, end)}
    open_list = [(manhattan_distance(start, end), 0, (start, (1, 0)))]
    visited = {}

    while open_list:
        f, cost, state = heapq.heappop(open_list)
        if state[0] == end:
            return cost

        if state in visited and visited[state] <= f:
            continue

        if state in open_dict:
            del open_dict[state]
        else:
            continue

        visited[state] = f
        pos, d = state

        for next_state, new_cost in next_states(pos, d, walls, cost):
            next_pos, _ = next_state
            n_f = new_cost + manhattan_distance(next_pos, end)
            if next_state in open_dict and n_f >= open_dict[next_state]:
                continue

            if next_state in visited and n_f >= visited[next_state]:
                continue

            open_dict[next_state] = n_f
            heapq.heappush(open_list, (n_f, new_cost, next_state))

    raise ValueError("No path found")


def part2(data: List[str]) -> Any:
    """ 2024 Day 16 Part 2
    >>> part2(["###############", "#.......#....E#", "#.#.###.#.###.#", "#.....#.#...#.#", "#.###.#####.#.#", "#.#.#.......#.#", "#.#.#####.###.#", "#...........#.#", "###.#.#####.#.#", "#...#.....#.#.#", "#.#.#.###.#.#.#", "#.....#...#.#.#", "#.###.#.#.#.#.#", "#S..#.....#...#", "###############"])
    45
    >>> part2(["#################", "#...#...#...#..E#", "#.#.#.#.#.#.#.#.#", "#.#.#.#...#...#.#", "#.#.#.#.###.#.#.#", "#...#.#.#.....#.#", "#.#.#.#.#.#####.#", "#.#...#.#.#.....#", "#.#.#####.#.###.#", "#.#.#.......#...#", "#.#.###.#####.###", "#.#.#...#.....#.#", "#.#.#.#####.###.#", "#.#.#.........#.#", "#.#.#.#########.#", "#S#.............#", "#################"])
    64
    """
    walls = set()
    start = None
    end = None
    for y, line in enumerate(data):
        for x, c in enumerate(line):
            if c == "S":
                start = (x, y)
            elif c == "E":
                end = (x, y)
            elif c == "#":
                walls.add((x, y))

    assert start is not None, "Start not found"
    assert end is not None, "End not found"

    open_dict = {(start, (1, 0)): manhattan_distance(start, end)}
    open_list = [(manhattan_distance(start, end), 0, (start, (1, 0)), {start})]
    visited = {}
    on_paths = defaultdict(set)

    while open_list:
        f, cost, state, on_path = heapq.heappop(open_list)
        on_path |= on_paths[state]
        if state[0] == end:
            # print_path(walls, on_path)
            return len(on_path)

        if state in visited and visited[state] < f:
            continue

        on_paths[state] |= on_path
        adding = False
        if state in open_dict:
            del open_dict[state]
            adding = True

        visited[state] = f
        pos, d = state

        for next_state, new_cost in next_states(pos, d, walls, cost):
            next_pos, _ = next_state
            n_f = new_cost + manhattan_distance(next_pos, end)
            if next_state in open_dict and n_f > open_dict[next_state]:
                continue

            if next_state in visited and n_f > visited[next_state]:
                continue

            if adding:
                open_dict[next_state] = n_f
                heapq.heappush(open_list, (n_f, new_cost, next_state, on_paths[state] | {next_pos}))
            else:
                on_paths[next_state] |= on_paths[state] | {next_pos}

    raise ValueError("No path found")



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
        print(f"\nPart 1:\nCost of shortest path: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nNumber of positions on all paths of shortest length: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return (p1, p1_time.elapsed), (p2, p2_time.elapsed)


if __name__ == "__main__":
    main(verbose=True)