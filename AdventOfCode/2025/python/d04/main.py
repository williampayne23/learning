import sys
from time import time

EXAMPLE_PUZZLE_INPUT = """..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."""
EXAMPLE_PUZZLE_INPUT_2 = None
EXAMPLE_1_SOLUTION = 13
EXAMPLE_2_SOLUTION = 43

import torch
import torch.nn.functional as F


def make_grid(input):
    grid = [[1 if char == "@" else 0 for char in line] for line in input.split("\n")]
    if len(grid[-1]) == 0:
        grid = grid[:-1]
    tensor = torch.tensor(grid, dtype=torch.float32)
    return tensor.unsqueeze(0).unsqueeze(0)


def get_removable_rolls(grid: torch.Tensor):
    kernel = torch.ones(1, 1, 3, 3)
    neighbor_count = F.conv2d(grid, kernel, padding=1) - grid
    fewer_than_4 = neighbor_count < 4
    removable_rolls = grid.logical_and(fewer_than_4)
    return removable_rolls


def solution_part_1(input):
    grid = make_grid(input)
    removable_rolls = get_removable_rolls(grid)
    return removable_rolls.sum().item()


def solution_part_2(input):
    grid = make_grid(input)
    zeros = grid * 0
    removable_rolls = get_removable_rolls(grid)
    removed = removable_rolls.sum().item()
    total_removed = removed
    while removed > 0:
        grid = torch.where(removable_rolls, zeros, grid)
        removable_rolls = get_removable_rolls(grid)
        removed = removable_rolls.sum().item()
        total_removed += removed
    return total_removed


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
