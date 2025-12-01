import sys

EXAMPLE_PUZZLE_INPUT = """\
"""
EXAMPLE_PUZZLE_INPUT_2 = None
EXAMPLE_1_SOLUTION = None
EXAMPLE_2_SOLUTION = None


def solution_part_1(input):
    pass


def solution_part_2(input):
    pass


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
    if EXAMPLE_PUZZLE_INPUT_2 is not None:
        assert solution_part_1(EXAMPLE_PUZZLE_INPUT) == EXAMPLE_1_SOLUTION
        assert solution_part_2(EXAMPLE_PUZZLE_INPUT_2) == EXAMPLE_2_SOLUTION
    else:
        assert solution_part_1(EXAMPLE_PUZZLE_INPUT) == EXAMPLE_1_SOLUTION
        assert solution_part_2(EXAMPLE_PUZZLE_INPUT) == EXAMPLE_2_SOLUTION
    print("All tests passed.")


if __name__ == "__main__":
    main()
