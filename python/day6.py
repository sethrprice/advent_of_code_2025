import numpy as np

with open("inputs/day6.txt") as f:
    lines = f.readlines()

grid = np.array([list(line.strip("\n")) for line in lines], dtype=str)

class Location:
    def __init__(self, loc, move):
        self.location = tuple(loc)
        self.move = tuple(move)
        
    def __repr__(self):
        return f"location: {self.location}, move: {self.move}\n"
    
    def __eq__(self, other):
        return self.location == other.location and self.move == other.move
    
    def __hash__(self):
        return hash((self.location, self.move))

def record_route(start, grid):
    loc = tuple(start)
    moves = [(-1,0), (0, 1), (1, 0), (0, -1)]
    move_i = 0
    move = moves[move_i]

    locations_visited = set()
    locations_visited.add(Location(loc, move))
    while True:
        try:
            next_loc = tuple(l + m for l, m in zip(loc, move))
            if next_loc[0] < 0 or next_loc[1] < 0:
                break
            next_obj = grid[next_loc]
            if next_obj == "#":
                move_i = (move_i + 1) % 4
                move = moves[move_i]
            else:
                loc = next_loc
            next_location = Location(loc, move)
        except:
            break
        if next_location not in locations_visited:
            locations_visited.add(next_location)
        else:
            return set()
        
    return locations_visited


# Part 1
start_tuple = list(np.asarray(grid == "^").nonzero())
start = tuple(map(lambda a: a.item(), start_tuple))

locations_visited = record_route(start, grid)

n_locations = len({l.location for l in locations_visited})

print(f"The guard visited {n_locations} unique locations.")


# Part 2
start_tuple = list(np.asarray(grid == "^").nonzero())
start = tuple(map(lambda a: a.item(), start_tuple))

loop_points = set()
start_location = Location(start, (-1, 0))
for location_i in locations_visited - {start_location}:
    obstacle_location = location_i.location
    try:
        original_value = grid[obstacle_location]
        if original_value == "#":
            continue
        grid[obstacle_location] = "#"
    except:
        continue
    route = record_route(start, grid)
    if len(route) > 0:
        grid[obstacle_location] = original_value
        continue
    else:
        print(f"putting obstacle at {obstacle_location}")
        loop_points.add(obstacle_location)
        grid[obstacle_location] = original_value
    
n_loop_points = len(loop_points)

print(f"There are {n_loop_points} locations that minimise time paradoxes")