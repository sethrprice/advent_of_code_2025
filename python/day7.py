with open("examples/day7.txt") as f:
    lines = f.readlines()

ll = [line.strip("\n").split(": ") for line in lines]

class Equation:
    def __init__(self, equation):
        self.lhs = equation[0]
        self.rhs = equation[1].split()

    def string_of(self):
        rhs_expression = " • ".join(self.rhs)
        return self.lhs + " = " + rhs_expression

    def __repr__(self):
        return self.string_of()
    
    def insert_operator(self, operators):
        string_of = self.string_of()
        for o in operators:
            try:
                index = string_of.index()
            except:
                break

equations = [Equation(line) for line in ll]

