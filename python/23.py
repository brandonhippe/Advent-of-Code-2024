import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from typing import Any, List, Tuple, Optional
from collections import defaultdict, deque
from itertools import product


def part1(data: List[str]) -> Any:
    """
    2024 Day 23 Part 1
    >>> part1(["kh-tc", "qp-kh", "de-cg", "ka-co", "yn-aq", "qp-ub", "cg-tb", "vc-aq", "tb-ka", "wh-tc", "yn-cg", "kh-ub", "ta-co", "de-co", "tc-td", "tb-wq", "wh-td", "ta-ka", "td-qp", "aq-cg", "wq-ub", "ub-vc", "de-ta", "wq-aq", "wq-vc", "wh-yn", "ka-de", "kh-ta", "co-tc", "wh-qp", "tb-vc", "td-yn"])
    7
    """
    connections = defaultdict(set)
    for line in data:
        a, b = line.split("-")
        connections[a].add(b)
        connections[b].add(a)
    total = set()
    for k, connected in connections.items():
        if k[0] != 't':
            continue

        for k1 in connected:
            for k2 in connections[k1]:
                if k2 in connected:
                    total.add(tuple(sorted([k, k1, k2])))

    return len(total)


def bron_kerbosch(connections: dict[str, set[str]], clique: set[str], candidates: set[str], excluded: set[str]):
    if not candidates and not excluded:
        return [clique]

    cliques = []
    pivot = max(candidates.union(excluded), key=lambda x: len(connections[x]))
    for v in list(candidates.difference(connections[pivot])):
        cliques.extend(bron_kerbosch(connections, clique | {v}, candidates & connections[v], excluded & connections[v]))
        candidates.remove(v)
        excluded.add(v)


    return cliques


def part2(data: List[str]) -> Any:
    """
    2024 Day 23 Part 2
    >>> part2(["kh-tc", "qp-kh", "de-cg", "ka-co", "yn-aq", "qp-ub", "cg-tb", "vc-aq", "tb-ka", "wh-tc", "yn-cg", "kh-ub", "ta-co", "de-co", "tc-td", "tb-wq", "wh-td", "ta-ka", "td-qp", "aq-cg", "wq-ub", "ub-vc", "de-ta", "wq-aq", "wq-vc", "wh-yn", "ka-de", "kh-ta", "co-tc", "wh-qp", "tb-vc", "td-yn"])
    'co,de,ka,ta'
    """   
    connections = defaultdict(set)
    for line in data:
        a, b = line.split("-")
        connections[a].add(b)
        connections[b].add(a)
    
    return ",".join(sorted(max(bron_kerbosch(connections, set(), set(connections.keys()), set()), key=lambda x: len(x))))


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
        print(f"\nPart 1:\nGroups of 3 interconnected pcs with one starting with t: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nLargest fully connected group: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return (p1, p1_time.elapsed), (p2, p2_time.elapsed)


if __name__ == "__main__":
    main(verbose=True)
