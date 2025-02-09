import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import heapq


def a_star(start, goal, corrupted, size):
    def manhat_dist(p1, p2):
        return sum(abs(c1 - c2) for c1, c2 in zip(p1, p2))

    open_list = [(manhat_dist(start, goal), 0, start)]
    open_dict = {start: manhat_dist(start, goal)}
    visited = {}

    while open_list:
        f, cost, pos = heapq.heappop(open_list)
        if pos == goal:
            return cost
        
        if pos in open_dict:
            del open_dict[pos]
        else:
            continue

        visited[pos] = f
        for offset in ((0, 1), (0, -1), (1, 0), (-1, 0)):
            new_pos = tuple(p + o for p, o in zip(pos, offset))
            if new_pos in corrupted or not (0 <= min(new_pos) and max(new_pos) <= size):
                continue

            nf = cost + 1 + manhat_dist(new_pos, goal)

            if new_pos in visited and visited[new_pos] <= nf:
                continue

            if new_pos in open_dict and open_dict[new_pos] <= nf:
                continue

            open_dict[new_pos] = nf
            heapq.heappush(open_list, (nf, cost + 1, new_pos))

    raise ValueError("No path found")


def part1(data: List[str], size=70, first=1024) -> Any:
    """ 2024 Day 18 Part 1
    >>> part1(["5,4", "4,2", "4,5", "3,0", "2,1", "6,3", "2,4", "1,5", "0,6", "3,3", "2,6", "5,1", "1,2", "5,5", "2,5", "6,5", "1,4", "0,4", "6,4", "1,1", "6,1", "1,0", "0,5", "1,6", "2,0"], 6, 12)
    22
    """
    corrupted = set()
    for i, line in enumerate(data):
        if i == first:
            break

        corrupted.add(tuple(map(int, line.split(','))))

    start = (0, 0)
    goal = (size, size)

    return a_star(start, goal, corrupted, size)


def part2(data: List[str], size=70) -> Any:
    """ 2024 Day 18 Part 2
    >>> part2(["5,4", "4,2", "4,5", "3,0", "2,1", "6,3", "2,4", "1,5", "0,6", "3,3", "2,6", "5,1", "1,2", "5,5", "2,5", "6,5", "1,4", "0,4", "6,4", "1,1", "6,1", "1,0", "0,5", "1,6", "2,0"], 6)
    '6,1'
    """

    corrupted = []
    for line in data:
        corrupted.append(tuple(map(int, line.split(','))))

    low, high = 0, len(corrupted)
    while high - low > 1:
        mid = (low + high) // 2
        start = (0, 0)
        goal = (size, size)

        try:
            a_star(start, goal, set(corrupted[:mid]), size)
            low = mid
        except ValueError:
            high = mid

    return ','.join(map(str, corrupted[low]))


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
        print(f"\nPart 1:\nShortest path after 1KB has fallen: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nPosition of first corrupted memory cell that blocks path: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return (p1, p1_time.elapsed), (p2, p2_time.elapsed)


if __name__ == "__main__":
    main(verbose=True)