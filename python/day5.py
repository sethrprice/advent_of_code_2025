import numpy as np

with open("inputs/day5.txt") as f:
    lines = f.readlines()

# Part 1
class Rule:
    def __init__(self, rule: str):
        self.first = int(rule.split("|")[0])
        self.last = int(rule.split("|")[1])
        self.original_rule = rule

    def __repr__(self) -> str:
        return self.original_rule
    
    def is_broken_by(self, update):
        if self.first not in update or self.last not in update:
            return False
        return np.where(update == self.first) > np.where(update == self.last)

rules = [Rule(rule.strip("\n")) for rule in lines if "|" in rule]
updates = [np.array(line.strip("\n").split(","), dtype=int) for line in lines if "," in line]

update_total = 0
for update in updates:
    for rule in rules:
        if rule.is_broken_by(update):
            break
    else:
        update_total += np.take(update, update.size // 2)
    
