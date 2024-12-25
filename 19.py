import re
from time import perf_counter
from functools import cache

with open("/home/stephan/Dev/aoc2024/data/inputs/19.txt", "r") as f:
    input_rows: list[str] = [x.strip() for x in f.readlines()]

bases_str = input_rows[0]
break_str = input_rows[1]
towels = input_rows[2:]

assert re.fullmatch(r"^((w|u|b|r|g)+, )+(w|u|b|r|g)+$", bases_str)
assert re.fullmatch(r"^$", break_str)
re_towel = re.compile(r"^(w|u|b|r|g)+$")
assert all(re.fullmatch(re_towel, x) for x in towels)

base_set = set(bases_str.split(", "))

@cache
def possible(x: str) -> bool:
    return x in base_set or any(possible(x[0:i]) and possible(x[i:]) for i in range(1, len(x)))


@cache
def num_ways(x: str) -> int:
    return (x in base_set) + sum(num_ways(x[i:]) for i in range(1, len(x)) if x[0:i] in base_set)


t_start = perf_counter()
print(sum(possible(x) for x in towels))
print(sum(num_ways(x) for x in towels))
t_end = perf_counter()
print(f"Time taken (s): {t_end - t_start}")
