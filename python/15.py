import re
import sys
from pathlib import Path
from typing import Any, List, Optional, Tuple

sys.path.append(str(Path(__file__).parent.parent.parent))
from Modules.timer import Timer
from collections import deque


def part1(data: List[str]) -> Any:
    """ 2024 Day 15 Part 1
    >>> part1(["##########", "#..O..O.O#", "#......O.#", "#.OO..O.O#", "#..O@..O.#", "#O#..O...#", "#O..O..O.#", "#.OO.O.OO#", "#....O...#", "##########", "", "<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^", "vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v", "><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<", "<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^", "^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><", "^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^", ">^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^", "<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>", "^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>", "v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"])
    10092
    """
    y = 0
    robot = None
    walls = set()
    boxes = set()
    while len(data[y]) != 0:
        for x, c in enumerate(data[y]):
            if c == '#':
                walls.add((x, y))
            elif c == '@':
                robot = (x, y)
            elif c == 'O':
                boxes.add((x, y))

        y += 1

    assert robot is not None, "No roboting position found"

    move_dirs = {'>': (1, 0), '<': (-1, 0), '^': (0, -1), 'v': (0, 1)}
    for line in data[y:]:
        for c in line:
            to_move = []
            pos = (robot[0] + move_dirs[c][0], robot[1] + move_dirs[c][1])
            while pos in boxes:
                to_move.append(pos)
                pos = (pos[0] + move_dirs[c][0], pos[1] + move_dirs[c][1])

            if pos in walls:
                continue

            if len(to_move) != 0:
                boxes.remove(to_move[0])
                boxes.add(pos)

            robot = (robot[0] + move_dirs[c][0], robot[1] + move_dirs[c][1])

    return sum(sum(c * m for c, m in zip(box, (1, 100))) for box in boxes)


def print_state_p2(walls, boxes, robot):
    x_min, x_max = min(x for x, _ in walls), max(x for x, _ in walls)
    y_min, y_max = min(y for _, y in walls), max(y for _, y in walls)

    for y in range(y_min, y_max + 1):
        for x in range(x_min, x_max + 1):
            if (x, y) in walls:
                print('#', end='')
            elif (x, x + 1, y) in boxes:
                print('[', end='')
            elif (x - 1, x, y) in boxes:
                print(']', end='')
            elif (x, y) == robot:
                print('@', end='')
            else:
                print('.', end='')

        print()


def part2(data: List[str]) -> Any:
    """ 2024 Day 15 Part 2
    >>> part2(["##########", "#..O..O.O#", "#......O.#", "#.OO..O.O#", "#..O@..O.#", "#O#..O...#", "#O..O..O.#", "#.OO.O.OO#", "#....O...#", "##########", "", "<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^", "vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v", "><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<", "<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^", "^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><", "^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^", ">^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^", "<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>", "^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>", "v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"])
    9021
    """
    y = 0
    robot = None
    walls = set()
    boxes = set()
    while len(data[y]) != 0:
        for x, c in enumerate(data[y]):
            if c == '#':
                walls.add((2 * x, y))
                walls.add((2 * x + 1, y))
            elif c == '@':
                robot = (2 * x, y)
            elif c == 'O':
                boxes.add((2 * x, 2 * x + 1, y))

        y += 1

    assert robot is not None, "No roboting position found"

    move_dirs = {'>': (1, 0), '<': (-1, 0), '^': (0, -1), 'v': (0, 1)}
    for line in data[y:]:
        for c in line:
            to_move = []
            x_off, y_off = move_dirs[c]
            if x_off == 0:
                # Vertical movement, check possible box alignments
                if (robot[0] - 1, robot[0], robot[1] + y_off) in boxes:
                    pushing = deque([(robot[0] - 1, robot[0], robot[1] + y_off)])
                elif (robot[0], robot[0] + 1, robot[1] + y_off) in boxes:
                    pushing = deque([(robot[0], robot[0] + 1, robot[1] + y_off)])
                else:
                    if (robot[0], robot[1] + y_off) not in walls:
                        robot = (robot[0], robot[1] + y_off)
                    continue

                to_move = []
                valid_move = True
                while pushing:
                    x1, x2, y = pushing.popleft()
                    if (x1, x2, y) in to_move:
                        continue

                    if (x1, y + y_off) in walls or (x2, y + y_off) in walls:
                        valid_move = False
                        break

                    to_move.append((x1, x2, y))
                    if (x1 - 1, x1, y + y_off) in boxes:
                        pushing.append((x1 - 1, x1, y + y_off))

                    if (x1, x2, y + y_off) in boxes:
                        pushing.append((x1, x2, y + y_off))

                    if (x2, x2 + 1, y + y_off) in boxes:
                        pushing.append((x2, x2 + 1, y + y_off))

                if not valid_move:
                    continue

                for box in to_move:
                    boxes.remove(box)

                for box in to_move:
                    boxes.add((box[0], box[1], box[2] + y_off))

                robot = (robot[0], robot[1] + y_off)
            else:
                # Horizontal movement
                pos = (min(robot[0] + x_off, robot[0] + 2 * x_off), max(robot[0] + x_off, robot[0] + 2 * x_off), robot[1])
                while pos in boxes:
                    to_move.append(pos)
                    pos = (pos[0] + 2 * x_off, pos[1] + 2 * x_off, pos[2])

                if ((pos[0], pos[2]) in walls and x_off == 1) or ((pos[1], pos[2]) in walls and x_off == -1):
                    continue

                for box in to_move:
                    boxes.remove(box)

                for box in to_move:
                    boxes.add((box[0] + x_off, box[1] + x_off, box[2]))

                robot = (robot[0] + x_off, robot[1])

    return sum(sum(c * m for c, m in zip((box[0], box[2]), (1, 100))) for box in boxes)


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
        print(f"\nPart 1:\nSum of box coordinates: {p1}\nRan in {p1_time.elapsed:0.4f} seconds")

    with Timer() as p2_time:
        p2 = part2(data)

    if verbose:
        print(f"\nPart 2:\nSum of box coordinates: {p2}\nRan in {p2_time.elapsed:0.4f} seconds")

    return (p1, p1_time.elapsed), (p2, p2_time.elapsed)


if __name__ == "__main__":
    main(verbose=True)