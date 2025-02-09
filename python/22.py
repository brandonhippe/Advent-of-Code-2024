import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from collections import defaultdict, deque
import multiprocessing


def next_secret_key(secret_key: int) -> int:
    secret_key ^= (secret_key) << 6
    secret_key &= 0xFFFFFF
    
    secret_key ^= (secret_key) >> 5
    secret_key &= 0xFFFFFF

    secret_key ^= (secret_key) << 11
    secret_key &= 0xFFFFFF

    return secret_key


def secret_key_worker_p1(secret_key: int) -> int:
    for _ in range(2000):
        secret_key = next_secret_key(secret_key)

    return secret_key


def part1(data: List[str]) -> Any:
    """ 2024 Day 22 Part 1
    >>> part1(["1", "10", "100", "2024"])
    37327623
    """

    return sum(multiprocessing.Pool().starmap(secret_key_worker_p1, [(int(line),) for line in data]))


def secret_key_worker_p2(secret_key: int) -> List[int]:
    last_digits = []
    for _ in range(2000):
        last_digits.append(secret_key % 10)
        secret_key = next_secret_key(secret_key)

    last_digits.append(secret_key % 10)
    return last_digits

def part2(data: List[str]) -> Any:
    """ 2024 Day 22 Part 2
    >>> part2(["1", "2", "3", "2024"])
    23
    """
    last_digits = multiprocessing.Pool().starmap(secret_key_worker_p2, [(int(line),) for line in data])
    four_sequences = defaultdict(int)
    for nums in last_digits:
        last_n = None
        diffs = deque(maxlen=4)

        buyer_sequences = {}
        for n in nums:
            if last_n is not None:
                diffs.append(n - last_n)
                if len(diffs) == 4:
                    d = tuple(diffs)
                    if d not in buyer_sequences:
                        buyer_sequences[d] = n

            last_n = n

        for k, v in buyer_sequences.items():
            four_sequences[k] += v

    return max(four_sequences.values())


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
        print(f"\nPart 1:\nSum of 2000th secret number for all buyers: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nMost bananas you can get: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return (p1, p1_time.elapsed), (p2, p2_time.elapsed)


if __name__ == "__main__":
    main(verbose=True)