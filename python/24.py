import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from collections import deque, defaultdict
from itertools import product, combinations
from functools import reduce
import operator
import re
from math import ceil, log2


def parse_data(data: List[str], to_swap: dict[str, str]) -> Tuple[dict[str, int], dict[tuple[str, str], list[tuple[str, str]]], dict[str, set[str]], set[str], dict[str, tuple[str, str]]]:
    in_init = True
    wires = {}
    operations = defaultdict(set)
    outputs = set()
    input_with = defaultdict(set)
    adj_list = {}

    for line in data:
        if not len(line):
            in_init = False
            continue

        if in_init:
            wires[line.split(': ')[0]] = int(line.split(': ')[1])
            continue
        else:
            operation, result = line.split(' -> ')
            if result in to_swap:
                result = to_swap[result]

            r1, op, r2 = operation.split(' ')
            operations[tuple(sorted((r1, r2)))].add((result, op))
            adj_list[result] = tuple(sorted((r1, r2)))

            if result[0] == 'z':
                outputs.add(result)

            input_with[r1].add(r2)
            input_with[r2].add(r1)

    return wires, operations, input_with, sorted(outputs), adj_list

def AND(a, b):
    if a == 0 or b == 0:
        return 0
    
    assert a != -1 and b != -1, "Wire values must exist for AND operation if not 0"
    return a & b

def OR(a, b):
    if a == 1 or b == 1:
        return 1

    assert a != -1 and b != -1, "Wire values must exist for OR operation if not 1"
    return a | b

def XOR(a, b):
    assert a != -1 and b != -1, "Wire values must exist for XOR operation"
    return a ^ b


def run_circuit(wires: dict[str, int], operations: dict[tuple[str, str], list[tuple[str, str]]], input_with: dict[str, set[str]]) -> dict[str, int]:
    wire_set = set(wires.keys())
    open_wires = deque({tuple(sorted((w1, w2))) for w1 in wire_set for w2 in wire_set if w1 in input_with[w2]})

    states = {}
    while open_wires:
        pair = open_wires.popleft()
        if pair not in operations:
            continue

        op_list = operations[pair]
        del operations[pair]

        r1, r2 = pair
        valid_results = []
        for result, op in op_list:
            try:
                wires[result] = globals()[op](wires.get(r1, -1), wires.get(r2, -1))
                wire_set.add(result)
                valid_results.append(result)
            except AssertionError:
                operations[pair].add((result, op))

        for result in valid_results:
            for w in input_with[result]:
                new_pair = tuple(sorted((result, w)))
                if w in wire_set and new_pair in operations:
                    open_wires.append(new_pair)
                    continue

                for _, op in operations[new_pair]:
                    if (op == 'AND' and wires[result] == 0) or (op == 'OR' and wires[result] == 1):
                        open_wires.append(new_pair)

    return wires


def part1(data: List[str]) -> Any:
    """ 
    2024 Day 24 Part 1
    >>> part1(["x00: 1", "x01: 1", "x02: 1", "y00: 0", "y01: 1", "y02: 0", "", "x00 AND y00 -> z00", "x01 XOR y01 -> z01", "x02 OR y02 -> z02"])
    4
    >>> part1(["x00: 1", "x01: 0", "x02: 1", "x03: 1", "x04: 0", "y00: 1", "y01: 1", "y02: 1", "y03: 1", "y04: 1", "", "ntg XOR fgs -> mjb", "y02 OR x01 -> tnw", "kwq OR kpj -> z05", "x00 OR x03 -> fst", "tgd XOR rvg -> z01", "vdt OR tnw -> bfw", "bfw AND frj -> z10", "ffh OR nrd -> bqk", "y00 AND y03 -> djm", "y03 OR y00 -> psh", "bqk OR frj -> z08", "tnw OR fst -> frj", "gnj AND tgd -> z11", "bfw XOR mjb -> z00", "x03 OR x00 -> vdt", "gnj AND wpb -> z02", "x04 AND y00 -> kjc", "djm OR pbm -> qhw", "nrd AND vdt -> hwm", "kjc AND fst -> rvg", "y04 OR y02 -> fgs", "y01 AND x02 -> pbm", "ntg OR kjc -> kwq", "psh XOR fgs -> tgd", "qhw XOR tgd -> z09", "pbm OR djm -> kpj", "x03 XOR y03 -> ffh", "x00 XOR y04 -> ntg", "bfw OR bqk -> z06", "nrd XOR fgs -> wpb", "frj XOR qhw -> z04", "bqk OR frj -> z07", "y03 OR x01 -> nrd", "hwm AND bqk -> z03", "tgd XOR rvg -> z12", "tnw OR pbm -> gnj"])
    2024
    """
    wires, operations, input_with, outputs, _ = parse_data(data, {})
    wires = run_circuit(wires, operations, input_with)
    return int(''.join(str(wires[w]) for w in sorted(outputs, reverse=True)), 2)


def part2(data: List[str], op: str='add') -> Any:
    """ 
    2024 Day 24 Part 2
    """
    def assign_input(wires, value, inputs):
        for i, w in enumerate(inputs):
            wires[w] = (value >> i) & 1

    def incorrect_on_paths(sh_amt, wires, operations, input_with, outputs, adj_list, get_all: bool=False):
        on_path: dict[int, dict[str, set]] = defaultdict(lambda: defaultdict(set))
        for t1, t2 in product(range(range_lim), repeat=2):
            in_out_slice = slice(sh_amt + ceil(log2(range_lim)) + 1)

            expected_val = op_func(t1 << sh_amt, t2 << sh_amt)
            expected_wires = {w: (expected_val >> i) & 1 for i, w in enumerate(outputs[in_out_slice])}

            wires = {}
            assign_input(wires, t1 << sh_amt, x_inputs[in_out_slice])
            assign_input(wires, t2 << sh_amt, y_inputs[in_out_slice])

            wires = run_circuit(wires, operations.copy(), input_with)

            incorrect_outputs = [w for w in outputs[in_out_slice] if w not in wires or wires[w] != expected_wires[w]]
            if len(incorrect_outputs) == 0:
                continue

            if not get_all:
                return [1]

            open_wires = deque([(w, w, expected_wires[w]) for w in incorrect_outputs])
            checked_wires = set()
            while open_wires:
                wire, dest_wire, exp = open_wires.popleft()
                if wire not in adj_list or wire in checked_wires:
                    continue

                checked_wires.add(wire)
                correct = wires.get(wire, -1) == exp
                on_path[wires.get(wire, -1)][dest_wire].add(wire)

                pair = adj_list[wire]
                pair_vals = [wires.get(w, -1) for w in pair]
                for r, op in operations[pair]:
                    if r != wire:
                        continue

                    for changed_vals in product(range(2), repeat=2):
                        try:
                            if not (correct ^ (globals()[op](*changed_vals) == exp)):
                                continue
                        except AssertionError:
                            if not correct:
                                continue

                        for w, p, c in zip(pair, pair_vals, changed_vals):
                            if p != c and w not in checked_wires:
                                open_wires.append((w, dest_wire, c))

        return on_path

    def get_swap_tuple(swaps):
        return tuple(sorted(swaps.keys()))

    def test_valid(max_sh_amt, wires, operations, input_with, outputs, adj_list):
        for sh_amt in range(max_sh_amt, max_sh_amt + 1):
            if len(incorrect_on_paths(sh_amt, wires, operations, input_with, outputs, adj_list)):
                return False

        return True

    try:
        op_func = dir(operator).index(op)
    except ValueError:
        op_func = dir(operator).index(f"__{op}__")

    op_func = getattr(operator, dir(operator)[op_func])

    swapped = None
    open_swaps = deque([({}, 0)])

    while open_swaps:
        to_swap, start_at = open_swaps.popleft()
        wires, operations, input_with, outputs, adj_list = parse_data(data, to_swap)
        outputs = sorted(outputs)
        x_inputs = sorted([w for w in input_with.keys() if w[0] == 'x'])
        y_inputs = sorted([w for w in input_with.keys() if w[0] == 'y'])

        found_error = False
        sh_amt = start_at
        while sh_amt < len(x_inputs):
            range_lim = 2 if sh_amt == len(x_inputs) - 1 else 4
            incorrect_outputs = incorrect_on_paths(sh_amt, wires, operations, input_with, outputs, adj_list, True)
            if len(incorrect_outputs) == 0 or (len(incorrect_outputs) < 2 and sh_amt < len(x_inputs) - 1):
                sh_amt += 1
                continue

            found_error = True
            dests = reduce(lambda x, y: x & y, (set(i_wires.keys()) for i_wires in incorrect_outputs.values()))
            dest_sets = [reduce(lambda x, y: x & y, (i_wires[d] for i_wires in incorrect_outputs.values() if d in i_wires)) for d in dests]
            
            valid_found = []
            for sets in combinations(dest_sets, min(len(dest_sets), 2)):
                repeat = 2 if len(sets) == 1 else 1
                for i_wire, j_wire in product(*sets, repeat=repeat):
                    if i_wire == j_wire or i_wire in to_swap or j_wire in to_swap:
                        continue

                    new_swap = {**to_swap}
                    new_swap[i_wire] = j_wire
                    new_swap[j_wire] = i_wire
                    
                    test_wires, test_operations, test_input_with, test_outputs, test_adj_list = parse_data(data, new_swap)
                    if not test_valid(sh_amt, test_wires, test_operations, test_input_with, test_outputs, test_adj_list):
                        continue
                    
                    valid_found.append(new_swap)

            if valid_found:
                open_swaps.extend((swap, sh_amt) for swap in valid_found)

            break

        if not found_error:
            swapped = get_swap_tuple(to_swap)
            break

    assert swapped is not None, "No valid swap(s) found"
    return ','.join(swapped)



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
        print(f"\nPart 1:\nOutput of circuit: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nWires involved in swaps to correct circuit operation: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return (p1, p1_time.elapsed), (p2, p2_time.elapsed)


if __name__ == "__main__":
    main(verbose=True)