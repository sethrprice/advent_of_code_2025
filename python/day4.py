import numpy as np

with open("inputs/day4.txt") as f:
    lines = f.readlines()

arr = np.array([list(line.strip('\n')) for line in lines])

# Part 1
xmas = ["X", "M", "A", "S"]

def rolling_window(a, size):
    shape = a.shape[:-1] + (a.shape[-1] - size + 1, size)
    strides = a.strides + (a.strides[-1],)
    return np.lib.stride_tricks.as_strided(a, shape=shape, strides=strides)

def count_xmas(a):
    count = 0
    for row in a:
        is_xmas = np.all(rolling_window(row, 4) == xmas, axis=1)
        count += is_xmas.sum()
    return count

def shift_arr_f(a):
    '''account for diagonal XMASs by shifting elements up by their index'''
    shifted_arr = np.empty(a.shape, dtype=str)
    for i, row in enumerate(a):
        rolled_row = np.roll(row, -i)
        if i != 0:
            rolled_row[-i:] = np.full(i, ".")
        shifted_arr[i] = rolled_row
    return shifted_arr.T

number_xmas = 0
for _j in range(4):
    arr = np.rot90(arr)
    shifted_arr_f = shift_arr_f(arr)
    shifted_arr_b = shift_arr_f(arr.T)[1:] # avoid double counting any XMASs in the leading diagonal

    v_and_h = count_xmas(arr)
    diag_1 = count_xmas(shifted_arr_f)
    diag_2 = count_xmas(shifted_arr_b)

    plus = v_and_h + diag_1 + diag_2

    number_xmas += plus

# Part 2
mas = np.array(["S", "A", "M", "S"])
n = len(arr[0])
jumps = np.array([2, n + 1, 2 * n, 2 * n + 2])

def count_x_mas(a):
    count = 0
    flat_arr = a.flatten()
    for i in np.where(flat_arr == "M")[0]:
        # if it starts at the end of a row it can't produce an X-MAS
        if i%n >= n - 2:
            continue
        indices = jumps + i
        if indices[-1] >= len(flat_arr):
            break
        if all(flat_arr[indices] == mas):
            count += 1
    return count

x_mas_count = 0
for _j in range(4):
    arr = np.rot90(arr)
    x_mas_count += count_x_mas(arr)

