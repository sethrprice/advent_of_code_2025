import numpy as np

with open("inputs/day1.txt") as f:
    lines = f.readlines()

# Part 1
arrs = np.array([line.split() for line in lines], dtype=int).T
arrs.sort()
total_diff = np.array([abs(a - b) for a, b in arrs.T]).sum()

# Part 2
similarity_score = 0
unique, counts = np.unique(arrs[1], return_counts=True)
dictionary = dict(zip(unique, counts))

for a in arrs[0]:
    if a in dictionary:
        plus = a * dictionary[a]
        similarity_score += plus