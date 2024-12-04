import numpy as np
import re

with open("examples/day3.txt") as f:
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
ex1 = " don't()_"

do = re.compile(r'do\(\)')
dont = re.compile(r'(?<!\w)don\'t\(\)')

d1 = dont.match(ex1)