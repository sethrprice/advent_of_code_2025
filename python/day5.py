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
    
    def is_in_update(self, update):
        return self.first in update.arr and self.last in update.arr
    
    def is_broken_by(self, update):
        if not self.is_in_update(update):
            return False
        return np.where(update.arr == self.first) > np.where(update.arr == self.last)
    
    def get_indices(self, update):
        return np.where(update.arr == self.first), np.where(update.arr == self.last)

class Update:
    def __init__(self, update, rules) -> None:
        self.arr = np.array(update, dtype=int)
        self.bad = False
        self.rules = [rule for rule in rules if rule.is_in_update(self)]

    def __repr__(self):
        return f"array: {self.arr} bad: {self.bad}"

    def middle(self):
        return np.take(self.arr, self.arr.size // 2)
    
    def flag_as_bad(self):
        self.bad = True

    def is_bad(self):
        return self.bad
    
    def fix_by(self, rule):
        ix1, ix2 = rule.get_indices(self)
        self.arr[[ix1, ix2]] = self.arr[[ix2, ix1]]
    


rules = [Rule(rule.strip("\n")) for rule in lines if "|" in rule]
updates = [Update(line.strip("\n").split(","), rules) for line in lines if "," in line]


update_total = 0
for update in updates:
    for rule in update.rules:
        if rule.is_broken_by(update):
            update.flag_as_bad()
            break
    if not update.is_bad():
        update_total += update.middle()
    

# Part 2


fixed_update_total = 0
for update in updates:
    i = 0
    rules_arr = update.rules
    while True:
        try:
            rule = rules_arr[i]
            if rule.is_broken_by(update):
                update.flag_as_bad()
                update.fix_by(rule)
                rules_arr.remove(rule)
                i = 0
            else:
                i += 1
        except:
            break
    if update.is_bad():
        fixed_update_total += update.middle()