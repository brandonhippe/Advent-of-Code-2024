import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
import re
from collections import deque


def run_program(program, registers):
    def combo_operand(op):
        if op >> 2 == 0:
            return op
        elif op != 7:
            return registers[chr(ord('A') + op - 4)]
        else:
            raise ValueError("Invalid operand")

    ip = 0
    output = []
    while 0 <= ip < len(program):
        ins, op = program[ip], program[ip + 1]

        if ins == 0:
            # adv
            registers['A'] >>= combo_operand(op)
        elif ins == 1:
            # bxl
            registers['B'] ^= op
        elif ins == 2:
            # bst
            registers['B'] = combo_operand(op) & 0x7
        elif ins == 3:
            # jnz
            if registers['A'] != 0:
                ip = op - 2
        elif ins == 4:
            # bxc
            registers['B'] ^= registers['C']
        elif ins == 5:
            # out
            output.append(combo_operand(op) & 0x7)
        elif ins == 6:
            # bdv
            registers['B'] = registers['A'] >> combo_operand(op)
        elif ins == 7:
            # cdv
            registers['C'] = registers['A'] >> combo_operand(op)
        else:
            raise ValueError("Invalid instruction")

        ip += 2

    return output


def part1(data: List[str]) -> Any:
    """ 2024 Day 17 Part 1
    >>> part1(["Register A: 729", "Register B: 0", "Register C: 0", "", "Program: 0,1,5,4,3,0"])
    '4,6,3,5,6,3,5,2,1,0'
    """

    registers = {}
    program = []
    for line in data:
        if len(line) == 0:
            continue

        if line.startswith("Register"):
            reg_c = line.split(" ")[1][:-1]
            reg_value = int(re.search(r'\d+', line).group())
            registers[reg_c] = reg_value
        else:
            program = [int(x) for x in re.findall(r'\d+', line)]

    assert len(program) != 0, "Program not found"
    return ",".join(map(str, run_program(program, registers)))


def part2(data: List[str]) -> Any:
    """ 2024 Day 17 Part 2
    >>> part2(["Register A: 2024", "Register B: 0", "Register C: 0", "", "Program: 0,3,5,4,3,0"])
    117440
    """
    
    registers = {}
    program = []
    for line in data:
        if len(line) == 0:
            continue

        if line.startswith("Register"):
            reg_c = line.split(" ")[1][:-1]
            reg_value = int(re.search(r'\d+', line).group())
            registers[reg_c] = reg_value
        else:
            program = [int(x) for x in re.findall(r'\d+', line)]

    assert len(program) != 0, "Program not found"

    # Build the possible A values from right to left of program
    matching = deque([((), 0)])
    checked = set()
    min_possible = float('inf')
    while len(matching) > 0:
        already_matched, pre_val = matching.popleft()
        if already_matched == tuple(program):
            min_possible = min(min_possible, pre_val)
            continue

        for n in range(2 ** 3):
            test_val = (pre_val << 3) + n
            if test_val in checked:
                continue

            checked.add(test_val)
            output = run_program(program, {'A': test_val, 'B': registers['B'], 'C': registers['C']})
            if tuple(program[-len(output):]) == tuple(output):
                matching.append((tuple(output), test_val))

    return min_possible


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
        print(f"\nPart 1:\nProgram output: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nMinimum value to make the program output itself: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return (p1, p1_time.elapsed), (p2, p2_time.elapsed)


if __name__ == "__main__":
    main(verbose=True)