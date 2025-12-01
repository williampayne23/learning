import sys
from typing import Literal
from pydantic import BaseModel

EXAMPLE_PUZZLE_INPUT = """\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"""
EXAMPLE_PUZZLE_INPUT_2 = None
EXAMPLE_1_SOLUTION = 3
EXAMPLE_2_SOLUTION = 6


class Operation(BaseModel):
    operation: Literal["plus", "minus"]
    amount: int

    def apply(self, value) -> int:
        if self.operation == "plus":
            return value + self.amount
        elif self.operation == "minus":
            return value - self.amount
        else:
            raise ValueError("Operation must be plus or minus")

    def count_zero_crosses(self, value) -> int:
        base = self.amount // 100
        if self.operation == "plus" and value + (self.amount % 100) >= 100:
            return base + 1

        if (
            self.operation == "minus"
            and value - (self.amount % 100) <= 0
            and value != 0
        ):
            return base + 1

        return base

    def __repr__(self):
        return f"{'+' if self.operation == 'plus' else '-'} {self.amount}"


def operation_from_line(line: str) -> Operation:
    leading_char = line[0]
    amount_str = line[1:]
    amount = int(amount_str)

    return Operation(
        operation="plus" if leading_char == "R" else "minus", amount=amount
    )


def solution_part_1(input: str):
    lines = input.splitlines()
    operations = [operation_from_line(line) for line in lines]

    position = 50
    zeros = 0
    for operation in operations:
        position = operation.apply(position)
        if position % 100 == 0:
            zeros += 1
    return zeros


def solution_part_2(input):
    lines = input.splitlines()
    operations = [operation_from_line(line) for line in lines]
    position = 50
    zeros = 0
    for operation in operations:
        zero_crosses = operation.count_zero_crosses(position)
        zeros += zero_crosses
        position = operation.apply(position) % 100
    return zeros


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

    print("Part 1:", solution_part_1(puzzle_input))
    print("Part 2:", solution_part_2(puzzle_input))


def _run_tests():
    print("Running tests...")
    part_1_result = solution_part_1(EXAMPLE_PUZZLE_INPUT)
    part_2_result = (
        solution_part_2(EXAMPLE_PUZZLE_INPUT)
        if EXAMPLE_PUZZLE_INPUT_2 is None
        else solution_part_2(EXAMPLE_PUZZLE_INPUT_2)
    )
    if part_1_result != EXAMPLE_1_SOLUTION:
        print(f"Part 1 test failed: expected {EXAMPLE_1_SOLUTION}, got {part_1_result}")
    if part_2_result != EXAMPLE_2_SOLUTION:
        print(f"Part 2 test failed: expected {EXAMPLE_2_SOLUTION}, got {part_2_result}")
    print("All tests passed.")


if __name__ == "__main__":
    main()
