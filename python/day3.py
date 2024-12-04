import numpy as np
import re

with open("inputs/day3.txt") as f:
    lines = f.readlines()

memory = "".join(lines)

mul = re.compile(r'(mul\(\d{1,3},\d{1,3}\))')

# Part 1
all_muls = mul.findall(memory)

def multiply_mul(s):
    new_s = s.replace("mul(", "")
    just_nums = new_s.replace(")", "")
    p = just_nums.partition(",")
    result = int(p[0]) * int(p[-1])
    return result

total = 0
for m in all_muls:
    mul_product = multiply_mul(m)
    total += mul_product


# Part 2
ex = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"

do = re.compile(r'do\(\)')
dont = re.compile(r"don\'t\(\)")

fixed_memory = ""
remaining_string = memory
stop_match = dont.search(remaining_string)

while stop_match is not None:
    fixed_memory = fixed_memory + remaining_string[:stop_match.start()]
    remaining_string = remaining_string[stop_match.end():]
    start_match = do.search(remaining_string)
    remaining_string = remaining_string[start_match.end():]
    stop_match = dont.search(remaining_string)

fixed_memory = fixed_memory + remaining_string

all_fixed_muls = mul.findall(fixed_memory)

total = 0
for m in all_fixed_muls:
    mul_product = multiply_mul(m)
    total += mul_product
