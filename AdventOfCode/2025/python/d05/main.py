import sys
from time import time
from pydantic import BaseModel
from functools import reduce

EXAMPLE_PUZZLE_INPUT = """3-5
10-14
16-20
12-18

1
5
8
11
17
32
"""
EXAMPLE_PUZZLE_INPUT_2 = None
EXAMPLE_1_SOLUTION = 3
EXAMPLE_2_SOLUTION = 14


class InclusiveRange(BaseModel):
    start: int
    end: int

    def check_includes(self, number: int) -> bool:
        return number >= self.start and number <= self.end

    def yield_numbers(self):
        for i in range(self.start, self.end + 1):
            yield i

    def can_combine(self, other_range: "InclusiveRange"):
        return not (
            self.end < other_range.start - 1 or other_range.end < self.start - 1
        )

    def combine(self, other_range: "InclusiveRange") -> "InclusiveRange":
        if not self.can_combine(other_range):
            raise Exception("This range cannot be combined")
        new_start = min(self.start, other_range.start)
        new_end = max(self.end, other_range.end)
        return InclusiveRange(start=new_start, end=new_end)

    def length(self):
        return self.end - self.start + 1


def parse_input(input: str):
    lines = input.split("\n")
    line = lines.pop()
    if line == "":
        line = lines.pop()
    numbers: list[int] = []
    while line != "":
        numbers.append(int(line))
        line = lines.pop()

    # Hit empty line time for the ranges
    line = lines.pop()
    ranges: list[InclusiveRange] = []
    while line != "":
        range_start_str, range_end_str = line.split("-")
        range_start = int(range_start_str)
        range_end = int(range_end_str)
        r = InclusiveRange(start=range_start, end=range_end)
        ranges.append(r)
        line = lines.pop() if len(lines) > 0 else ""

    # Compress ranges
    # Sort by start,
    ranges = sorted(ranges, key=lambda r: r.start)
    compressed_ranges = []
    current_range = ranges[0]
    for r in ranges[1:]:
        if current_range.can_combine(r):
            current_range = current_range.combine(r)
        else:
            compressed_ranges.append(current_range)
            current_range = r
    compressed_ranges.append(current_range)
    return compressed_ranges, numbers


def solution_part_1(input):
    ranges, numbers = parse_input(input)

    def reducer(acc, v):
        if any([r.check_includes(v) for r in ranges]):
            return acc + 1
        return acc

    total = reduce(reducer, numbers, 0)
    return total


def solution_part_2(input):
    ranges, _ = parse_input(input)
    total = reduce(lambda total, r: total + r.length(), ranges, 0)

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
    ("All tests passed.")


if __name__ == "__main__":
    main()
