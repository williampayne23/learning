import sys
from time import time
from pydantic import BaseModel

EXAMPLE_PUZZLE_INPUT = """11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"""
EXAMPLE_PUZZLE_INPUT_2 = None
EXAMPLE_1_SOLUTION = 1227775554
EXAMPLE_2_SOLUTION = 4174379265


class IDRange(BaseModel):
    start: int
    end: int

    @staticmethod
    def from_str(range_str: str) -> "IDRange":
        [start_str, end_str] = range_str.split("-")
        start = int(start_str)
        end = int(end_str)
        return IDRange(start=start, end=end)

    def dist(self):
        return self.end - self.start

    def invalid_ids(self):
        return [i for i in range(self.start, self.end + 1) if is_invalid_id(i)]

    def fast_invalid_ids(self):
        # Need to loop through ids, skip immedietly to ids which could be invalid
        # E.g if range is from 123100 to 123300 we know that only 123123 is invalid
        # In general for ABC??? the next valid number is either ABCABC or ABDABD.

        # Get prefix (the first half of the number as a number)
        length = len(str(self.start))
        if length % 2 != 0:
            prefix = round(10 ** (length // 2))
        else:
            prefix = round(self.start // (10 ** (length / 2)))
        # Current number is prefix concatenated with prefix
        current = prefix * (10 ** len(str(prefix))) + prefix
        while current >= self.start and current <= self.end:
            # yield current whilst iterating the prefix provided we stay
            # within range
            yield current
            prefix += 1
            current = prefix * (10 ** len(str(prefix))) + prefix

    def invalid_ids_2(self):
        return [i for i in range(self.start, self.end + 1) if is_invalid_id_2(i)]

    def fast_invalid_ids_2(self):
        # basically do what was done in fast ids 1 but for different prefix lengths
        # Except that doesn't work because we don't know how many times to repeat the prefix?
        # I guess just repeat enough times to be > self.start then do passes for more repeats if needed?
        half_len = len(str(self.end)) // 2
        vals = set()
        for i in range(1, half_len + 1):
            added = list(self.fast_invalid_ids_fixed_prefix(i))
            vals = vals.union(added)
        return list(vals)

    def fast_invalid_ids_fixed_prefix(self, prefix_len: int):
        # for number abcdef____
        start_len = len(str(self.start))
        end_len = len(str(self.end))
        # get number prefix
        start_prefix = 10 ** (prefix_len - 1)
        end_prefix = 10 ** (prefix_len)
        repeats_to_try = {start_len // prefix_len, end_len // prefix_len}

        for repeats in repeats_to_try:
            if repeats <= 1:
                continue
            for prefix in range(start_prefix, end_prefix):
                current = repeat_prefix(prefix, repeats)
                if current > self.start and current < self.end:
                    yield current


def repeat_prefix(prefix: int, repeats: int):
    val: int = 0
    prefix_len = len(str(prefix))
    for i in range(repeats):
        val += prefix * 10 ** (prefix_len * i)
    return val


def is_invalid_id(id: int) -> bool:
    str_id = str(id)
    if len(str_id) % 2 != 0:
        return False

    half_way_point = len(str_id) // 2
    for i in range(half_way_point):
        if str_id[i] != str_id[i + half_way_point]:
            return False
    return True


def is_invalid_id_2(id: int) -> bool:
    # Evaluates if the starting string ever repeats
    # So 123123123 is valid
    # 12123 is not valid
    # 11111 is valid
    # 121212 is valid

    # One thing to try:
    # Brute force try increasingly large substrings once you get to the half length of the number give up.
    str_id = str(id)
    length = len(str_id)
    for len_check in range(1, length // 2 + 1):
        if length % len_check != 0:
            continue
        s = set(str_id[i : i + len_check] for i in range(0, length, len_check))
        if len(s) == 1:
            return True
    return False


def parse_ip_ranges(input: str):
    range_strings = input.split(",")
    return [IDRange.from_str(s) for s in range_strings]


def solution_part_1(input: str):
    ranges = parse_ip_ranges(input)
    invalid_ids = [invalid_id for r in ranges for invalid_id in r.fast_invalid_ids()]
    return sum(invalid_ids)


def solution_part_2(input):
    ranges = parse_ip_ranges(input)
    invalid_ids = [invalid_id for r in ranges for invalid_id in r.fast_invalid_ids_2()]
    assert sum(invalid_ids) == 48631958998
    return sum(invalid_ids)


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
