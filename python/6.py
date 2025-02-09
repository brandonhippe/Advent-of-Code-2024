import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from typing import List, Tuple, Set, Generator, Optional, Any
from multiprocessing import Pool


def parse_input(data: List[str]) -> Tuple[Tuple[int,], Tuple[int,], Set[Tuple[int,]]]:
    walls = set()
    pos = None
    curr_dir = None
    for y, line in enumerate(data):
        for x, char in enumerate(line):
            if char == '#':
                walls.add((x, y))
            elif char != '.':
                pos = (x, y)
                curr_dir = {'^': (0, -1), 'v': (0, 1), '<': (-1, 0), '>': (1, 0)}[char]

    assert pos is not None and curr_dir is not None, "No starting position found"
    return pos, curr_dir, walls


def guard_movement(pos: Tuple[int,], curr_dir: Tuple[int,], walls: Set[Tuple[int,]], dims: Tuple[int,], visited: Optional[Set[Tuple[int,]]]=None) -> Generator[Tuple[Tuple[int,],], None, bool]:
    while all(0 <= pos[i] < dims[i] for i in range(len(dims))):
        if visited is not None:
            if (*pos, *curr_dir) in visited:
                return True
            
            visited.add((*pos, *curr_dir))

        yield pos, curr_dir
        while True:
            next_pos = tuple(map(sum, zip(pos, curr_dir)))
            if next_pos not in walls:
                pos = next_pos
                break

            curr_dir = -curr_dir[1], curr_dir[0]

    return False


def run_until_return(generator_func, *args):
    gen = generator_func(*args)
    try:
        while True:
            next(gen)
    except StopIteration as e:
        return e.value


def part1(data: List[str]) -> Any:
    """ 2024 Day 6 Part 1
    >>> part1(["....#.....", ".........#", "..........", "..#.......", ".......#..", "..........", ".#..^.....", "........#.", "#.........", "......#..."])
    41
    """
    pos, curr_dir, walls = parse_input(data)
    return len(set(g[0] for g in guard_movement(pos, curr_dir, walls, (len(data[0]), len(data)))))


def part2(data: List[str]) -> Any:
    """ 2024 Day 6 Part 2
    >>> part2(["....#.....", ".........#", "..........", "..#.......", ".......#..", "..........", ".#..^.....", "........#.", "#.........", "......#..."])
    6
    """

    start_pos, start_dir, walls = parse_input(data)
    checked = {start_pos: (start_dir, set())}
    visited_dirs = set()

    for curr_pos, curr_dir in guard_movement(start_pos[:], start_dir[:], walls, (len(data[0]), len(data))):
        if not (curr_pos == start_pos and curr_dir == start_dir and len(visited_dirs) == 0):
            visited_dirs.add((*curr_pos, *curr_dir))

        if curr_pos in checked or len(visited_dirs) == 0:
            continue

        checked[curr_pos] = (curr_dir, visited_dirs.copy())

    del checked[start_pos]
    p = Pool()
    obstructions = p.starmap(run_until_return, ((guard_movement, tuple(p - d for p, d in zip(pos, di)), (-di[1], di[0]), walls.copy().union({pos}), (len(data[0]), len(data)), vi) for pos, (di, vi) in checked.items()))
    return sum(obstructions)


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
        print(f"\nPart 1:\nNumber of positions visited: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nNumber of obstacle positions that make a loop: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return (p1, p1_time.elapsed), (p2, p2_time.elapsed)


if __name__ == "__main__":
    main(verbose=True)