from itertools import product
from operator import add, mul

with open("inputs/day7.txt") as f:
    lines = f.readlines()

ll = [line.strip("\n").split(": ") for line in lines]

class Equation:
    def __init__(self, equation):
        self.lhs = int(equation[0])
        self.rhs = [int(n) for n in equation[1].split()]

    def string_of(self):
        rhs_expression = " • ".join(str(n) for n in self.rhs)
        return str(self.lhs) + " = " + rhs_expression

    def __repr__(self):
        return self.string_of()

equations = [Equation(line) for line in ll]

def digits(n):
    return len(str(n))

def endswith(a, b):
    return (a - b) % 10 ** digits(b) == 0

def works(lhs, rhs, check_concat=False):
    *head, n = rhs
    if not head:
        return n == lhs
    q, r = divmod(lhs, n)
    if r == 0 and works(q, head, check_concat):
        return True
    if check_concat and endswith(lhs, n) and works(lhs // (10 ** digits(n)), head, check_concat):
        return True
    return works(lhs - n, head, check_concat)

check = 0
for eq in equations:
    if works(eq.lhs, eq.rhs):
        check += eq.lhs

def get_calibration_number(equations, check_concat=False):
    sum_of_test_values = 0
    for eq in equations:
        if works(eq.lhs, eq.rhs, check_concat):
            sum_of_test_values += eq.lhs

        # number_of_operators = len(eq.rhs) - 1
        # op_combos = product(operator_map.values(), repeat=number_of_operators)
        # for op_combo in op_combos:
        #     result = eq.rhs[0]
        #     for i, op in enumerate(op_combo):
        #         result = op(result, eq.rhs[i+1])
        #     if result == eq.lhs:
        #         sum_of_test_values += eq.lhs
        #         break
    return sum_of_test_values



# Part 1

operator_map_1 = {
    "+": add, 
    "*": mul, 
}

calibration_1 = get_calibration_number(equations)

print(f"The total calibration result (part 1) is {calibration_1}.")




# Part 2

def concat(a, b):
    return(int(str(a) + str(b)))

operator_map_2 = {
    "+": add, 
    "*": mul, 
    "||": concat
}

calibration_2 = get_calibration_number(equations, True)

print(f"The total calibration result (part 2) is {calibration_2}.")