import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re


class Rule:
    def __init__(self, rule: str):
        self.rule = int(rule)
        self.before = set()

    def __hash__(self) -> int:
        return hash(self.rule)
    
    def __lt__(self, other):
        return other.rule in self.before
    
    def __eq__(self, value: int) -> bool:
        if isinstance(value, int):
            return self.rule == value
        
        return self.rule == value.rule
    
    def __repr__(self):
        return f"{self.rule}"
    
    def add(self, other):
        self.before.add(other)


def ordering(data: List[str], p2: bool=False) -> int:
    ordering_rules = {}
    total = 0
    for line in data:
        str_match = re.match(r"(\d+)\|(\d+)", line)
        if str_match:
            a, b = map(Rule, str_match.groups(range(1,3)))
            if a in ordering_rules:
                a = ordering_rules[a]
            else:
                ordering_rules[a] = a

            if b in ordering_rules:
                b = ordering_rules[b]
            else:
                ordering_rules[b] = b

            ordering_rules[a].add(b)
        elif len(line):
            int_line = list(ordering_rules[Rule(x)] for x in line.split(',') if int(x) in ordering_rules)
            sorted_line = sorted(int_line)
            
            if p2 ^ all(a == b for a, b in zip(int_line, sorted_line)):
                total += sorted_line[len(sorted_line) // 2].rule

    return total


def part1(data: List[str]) -> Any:
    """ 2024 Day 5 Part 1
    >>> part1(["47|53", "97|13", "97|61", "97|47", "75|29", "61|13", "75|53", "29|13", "97|29", "53|29", "61|53", "97|53", "61|29", "47|13", "75|47", "97|75", "47|61", "75|61", "47|29", "75|13", "53|13", "", "75,47,61,53,29", "97,61,53,29,13", "75,29,13", "75,97,47,61,53", "61,13,29", "97,13,75,29,47"])
    143
    """
    return ordering(data)


def part2(data: List[str]) -> Any:
    """ 2024 Day 5 Part 2
    >>> part2(["47|53", "97|13", "97|61", "97|47", "75|29", "61|13", "75|53", "29|13", "97|29", "53|29", "61|53", "97|53", "61|29", "47|13", "75|47", "97|75", "47|61", "75|61", "47|29", "75|13", "53|13", "", "75,47,61,53,29", "97,61,53,29,13", "75,29,13", "75,97,47,61,53", "61,13,29", "97,13,75,29,47"])
    123
    """
    return ordering(data, True)


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
        print(f"\nPart 1:\nSum of correctly ordered middle values: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nSum of corrected order middle values: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return (p1, p1_time.elapsed), (p2, p2_time.elapsed)


if __name__ == "__main__":
    main(verbose=True)