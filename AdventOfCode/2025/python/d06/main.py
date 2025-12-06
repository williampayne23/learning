from functools import reduce
import sys
from time import time
from typing import Tuple

EXAMPLE_PUZZLE_INPUT = """123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
"""
EXAMPLE_PUZZLE_INPUT_2 = None
EXAMPLE_1_SOLUTION = 4277556
EXAMPLE_2_SOLUTION = 3263827


def parse_item(input: str) -> Tuple[str, str | None]:
    start = 0
    while start < len(input):
        if input[start] == " ":
            start += 1
            continue
        break

    if start == len(input):
        return "", None
    end = start
    while end < len(input):
        if input[end] != " ":
            end += 1
            continue
        break

    item = input[start:end]
    new_str = input[end:]
    return new_str, item


def parse_until_number(input: str) -> Tuple[str, int | None]:
    new_str, item = parse_item(input)

    if item is None:
        return new_str, item

    num = int(item)
    return new_str, num


def parse_line(input: str):
    items: list[str] = []
    while True:
        input, item = parse_item(input)
        if item is None:
            break
        items.append(item)
    return items


def parse_number_line(input: str):
    items = parse_line(input)
    nums = [int(item) for item in items]
    return nums


def add(a, b):
    return a + b


def mult(a, b):
    return a * b


def solution_part_1(input):
    lines = input.split("\n")
    lines.pop()  # newline at the end of input
    operator_line = lines.pop()
    operators = [add if op == "+" else mult for op in parse_line(operator_line)]
    numbers = [parse_number_line(line) for line in lines]
    totals = numbers[0]
    for row in numbers[1:]:
        for i, op in enumerate(operators):
            totals[i] = op(totals[i], row[i])

    return sum(totals)


def get_single_char_column(lines, i):
    return [line[i] for line in lines]


def solve_one_column(lines, i):
    col = get_single_char_column(lines, i)
    operator = col.pop()
    assert operator == "+" or operator == "*", (
        f"Operator invalid! got {operator}, i = {i}"
    )
    col = int("".join(col))
    cols = [col]

    i += 1
    while i < len(lines[0]):
        col = get_single_char_column(lines, i)
        col.pop()  # pop operator line
        col = "".join(col).strip()
        i += 1
        if col == "":
            break
        cols.append(int(col))
    return i, reduce(add if operator == "+" else mult, cols)


def solution_part_2(input):
    lines = input.split("\n")
    lines.pop()  # pop newline
    length = len(lines[0])
    i = 0
    total = 0
    while i < length:
        i, answer = solve_one_column(lines, i)
        total += answer
    return total


def main():
    with open("input.txt") as f:
        puzzle_input = f.read()
    if sys.argv[-1] == "test":
        _run_tests()
        return

    if puzzle_input is None or puzzle_input == "":
        print("No input provided.")
        _run_tests()
        return

    start_time = time()
    print("Part 1:", solution_part_1(puzzle_input))
    print(f"took {time() - start_time:.4f}s")
    start_time = time()
    print("Part 2:", solution_part_2(puzzle_input))
    print(f"took {time() - start_time:.4f}s")


def _run_tests():
    print("Running tests...")
    if EXAMPLE_PUZZLE_INPUT_2 is not None:
        assert solution_part_1(EXAMPLE_PUZZLE_INPUT) == EXAMPLE_1_SOLUTION
        assert solution_part_2(EXAMPLE_PUZZLE_INPUT_2) == EXAMPLE_2_SOLUTION
    else:
        assert solution_part_1(EXAMPLE_PUZZLE_INPUT) == EXAMPLE_1_SOLUTION
        assert solution_part_2(EXAMPLE_PUZZLE_INPUT) == EXAMPLE_2_SOLUTION
    print("All tests passed.")


if __name__ == "__main__":
    main()
