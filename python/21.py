import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re
from itertools import product
from functools import cache


@cache
def shortest_sequence(key: str, start_key: str, num_directional: int, num_keypad: int=0) -> int:
    if num_keypad == 0:
        keypad_paths = NUM_KEYPAD_PATHS
    elif num_keypad <= num_directional:
        keypad_paths = DIR_KEYPAD_PATHS
    else:
        return 1

    new_keycodes = {f"{s}A" for s in keypad_paths[start_key][key]}
    shortest_path = float('inf')
    for keycode in new_keycodes:
        path_len = 0
        start = 'A'
        for c in keycode:
            path_len += shortest_sequence(c, start, num_directional, num_keypad + 1)
            start = c

        shortest_path = min(shortest_path, path_len)

    assert shortest_path != float('inf'), f"Shortest path not found for {key} from {start_key}"
    return shortest_path


def floyd_warshall(keypad_paths: dict) -> dict:
    for k in keypad_paths.keys():
        for i in keypad_paths.keys():
            for j in keypad_paths.keys():
                if i == j:
                    keypad_paths[i][j] = {''}
                
                i_k_paths = (keypad_paths[i][k] if k in keypad_paths[i] else ['0' * 100])
                k_j_paths = (keypad_paths[k][j] if j in keypad_paths[k] else ['0' * 100])
                for test_str in [i_k + k_j for i_k, k_j in product(i_k_paths, k_j_paths)]:
                    if j not in keypad_paths[i] or len(test_str) < len(list(keypad_paths[i][j])[0]):
                        keypad_paths[i][j] = {test_str}
                    elif len(test_str) == len(list(keypad_paths[i][j])[0]):
                        keypad_paths[i][j].add(test_str)

    return keypad_paths


NUM_KEYPAD_PATHS = floyd_warshall({
    '0': {'2': {'^'}, 'A': {'>'}},
    '1': {'2': {'>'}, '4': {'^'}},
    '2': {'1': {'<'}, '3': {'>'}, '5': {'^'}, '0': {'v'}},
    '3': {'2': {'<'}, '6': {'^'}, 'A': {'v'}},
    '4': {'1': {'v'}, '5': {'>'}, '7': {'^'}},
    '5': {'2': {'v'}, '4': {'<'}, '6': {'>'}, '8': {'^'}},
    '6': {'3': {'v'}, '5': {'<'}, '9': {'^'}},
    '7': {'4': {'v'}, '8': {'>'}},
    '8': {'5': {'v'}, '7': {'<'}, '9': {'>'}},
    '9': {'6': {'v'}, '8': {'<'}},
    'A': {'3': {'^'}, '0': {'<'}}
})

DIR_KEYPAD_PATHS = floyd_warshall({
    '^': {'A': {'>'}, 'v': {'v'}},
    '<': {'v': {'>'}},
    'v': {'<': {'<'}, '^': {'^'}, '>': {'>'}},
    '>': {'A': {'^'}, 'v': {'<'}},
    'A': {'^': {'<'}, '>': {'v'}}
})


def part1(data: List[str]) -> Any:
    """ 2024 Day 21 Part 1
    >>> part1(["029A", "980A", "179A", "456A", "379A"])
    126384
    """
    complexity = 0
    for keycode in data:
        start_pos = 'A'
        presses = 0
        for c in keycode:
            presses += shortest_sequence(c, start_pos, 2)
            start_pos = c

        complexity += presses * int(re.search(r"\d+", keycode).group())

    return complexity


def part2(data: List[str]) -> Any:
    """ 2024 Day 21 Part 2
    """
    complexity = 0
    for keycode in data:
        start_pos = 'A'
        presses = 0
        for c in keycode:
            presses += shortest_sequence(c, start_pos, 25)
            start_pos = c

        complexity += presses * int(re.search(r"\d+", keycode).group())

    return complexity


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
        print(f"\nPart 1:\nTotal complexity with 2 intermediate robots: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nTotal complexity with 25 intermediate robots: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return (p1, p1_time.elapsed), (p2, p2_time.elapsed)


if __name__ == "__main__":
    main(verbose=True)