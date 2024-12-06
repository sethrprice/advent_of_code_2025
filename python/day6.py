import numpy as np

with open("inputs/day6.txt") as f:
    lines = f.readlines()

def inside_bounds(location, grid):
    return location[0].item() > 0 and location[0].item() < grid.shape[0] and location[1].item() > 0 and location[1].item() < grid.shape[1]

grid = np.array([list(line.strip("\n")) for line in lines], dtype=str)

start_tuple = np.asarray(grid == "^").nonzero()
start = np.array(
    list(
        map(lambda a: a.item(), start_tuple)
    )
)

loc = start.copy()
current_obj = grid[tuple(loc)]
move = np.array([-1,0])
rot90 = np.array([[0, 1],[-1, 0]])

locations_visited = []
while True:
    locations_visited.append(complex(loc[0], loc[1]))
    loc += move
    try:
        current_obj = grid[tuple(loc)]
    except:
        break
    if current_obj == "#":
        loc -= move
        move = np.matmul(rot90, move)

len(set(locations_visited))