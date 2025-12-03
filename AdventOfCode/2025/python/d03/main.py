import sys
from time import time

EXAMPLE_PUZZLE_INPUT = """987654321111111
811111111111119
234234234234278
818181911112111"""
EXAMPLE_PUZZLE_INPUT_2 = None
EXAMPLE_1_SOLUTION = 357
EXAMPLE_2_SOLUTION = 3121910778619


def process_line(line: str, length: int):
    digits = [int(c) for c in line]
    if len(digits) == 0:
        return 0
    return recursive_make_joltage(digits, length)


def recursive_make_joltage(digits, length) -> int:
    if length == 0:
        return 0
    if len(digits) < length:
        raise Exception("Not enough digits for length")

    max_dig = 0
    max_joltage = 0
    i_to_iterate_to = len(digits) - length

    for i, digit in enumerate(digits):
        if i > i_to_iterate_to:
            break
        if digit > max_dig:
            max_joltage = digit * (10 ** (length - 1)) + recursive_make_joltage(
                digits[i + 1 :], length - 1
            )
            max_dig = digit
    return max_joltage


def solution_part_1(input):
    line_res = [process_line(line, 2) for line in input.split("\n")]
    return sum(line_res)


def solution_part_2(input):
    line_res = [process_line(line, 12) for line in input.split("\n")]
    return sum(line_res)


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
