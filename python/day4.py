import numpy as np

with open("examples/day4.txt") as f:
    lines = f.readlines()

arr = np.array([list(line.strip('\n')) for line in lines])

xmas = ["X", "M", "A", "S"]

def rolling_window(a, size):
    shape = a.shape[:-1] + (a.shape[-1] - size + 1, size)
    strides = a.strides + (a.strides[-1],)
    return np.lib.stride_tricks.as_strided(a, shape=shape, strides=strides)

def count_xmas(a):
    count = 0
    for _j in range(4):
        a = np.rot90(a) 
        for row in a:
            is_xmas = np.all(rolling_window(row, 4) == xmas, axis=1)
            count += is_xmas.sum()
    return count

count_xmas(arr)

number_xmas = 0



shifted_arr = np.empty(arr.shape, dtype=str)
for i, row in enumerate(arr):
    rolled_row = np.roll(row, -i)
    if i != 0:
        rolled_row[-i:] = np.full(i, ".")
    shifted_arr[i] = rolled_row

for i, row in enumerate(arr):
    rolled_row = np.roll(row, i)
    if i != 0:
        rolled_row[:i] = np.full(i, ".")
    shifted_arr[i] = rolled_row

