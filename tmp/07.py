from math import log10
from operator import add, mul


def concat_ints(x, y):
    return x * (10 ** (int(log10(y)) + 1)) + y  # int(str(x) + str(y))


def is_solvable(target, nums, operators):
    accumulated = [nums[0]]

    # don't include last number
    for num in nums[1:-1]:
        accumulated = [
            result
            for partial_result in accumulated
            for op in operators
            if (result := op(partial_result, num)) <= target
        ]

    # another explicit round for the last number so we can stop early
    for partial_result in accumulated:
        for op in operators:
            if op(partial_result, nums[-1]) == target:
                return True

    return False


def main():
    calibration_part1, calibration_part2 = 0, 0

    with open("/home/stephan/Dev/aoc2024/data/inputs/07.txt", "rt") as f:
        ii =0
        for line in f:
            test_val, numbers = line.split(":")
            test_val = int(test_val)
            numbers = list(map(int, numbers.split()))

            if is_solvable(test_val, numbers, [mul, add]):
                calibration_part1 += test_val
                print(f"{ii} -- {test_val} -- {calibration_part1}")
            else:
                if is_solvable(test_val, numbers, [mul, add, concat_ints]):
                    calibration_part2 += test_val
            ii = ii + 1

    print(f"Part 1: {calibration_part1}")
    print(f"Part 2: {calibration_part1+calibration_part2}")


if __name__ == "__main__":
    main()

