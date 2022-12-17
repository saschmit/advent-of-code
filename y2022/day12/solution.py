#!/usr/bin/python

import sys

class Cell:
    def __init__(self, letter):
        self.elev = ord(letter) - ord('a')
        self.steps = None

class ElMap:
    def __init__(self, grid):
        self.grid = grid
        self.height = len(grid)
        self.width = len(grid[0])
    def get(self, pos):
        row, col = pos
        return self.grid[row][col]

def load_input(filename):
    grid = []
    start = None
    end = None
    r = 0
    c = 0
    for line in open(filename):
        line = line.strip()
        row = []
        for letter in line:
            if letter == 'S':
                row.append(Cell('a'))
                start = (r, c)
            elif letter == 'E':
                row.append(Cell('z'))
                end = (r, c)
            else:
                row.append(Cell(letter))
            c += 1
        grid.append(row)
        r += 1
        c = 0
    return ElMap(grid), start, end

def gen_shortest_paths(elmap, end):
    def move_generator(height, width, current):
        # Up
        if current[0] > 0:
            yield (current[0]-1, current[1])

        # Down
        if current[0] < height-1:
            yield (current[0]+1, current[1])

        # Left
        if current[1] > 0:
            yield (current[0], current[1]-1)

        # Right
        if current[1] < width-1:
            yield (current[0], current[1]+1)

    elmap.get(end).steps = 0

    queue = [end]
    while queue:
        to_pos = queue.pop(0)
        to_cell = elmap.get(to_pos)
        assert elmap.get(to_pos).steps is not None
        for from_pos in move_generator(elmap.height, elmap.width, to_pos):
            from_cell = elmap.get(from_pos)
            if to_cell.elev - from_cell.elev > 1:
                continue
            if from_cell.steps is not None:
                continue

            if from_cell.steps is None:
                from_cell.steps = to_cell.steps + 1
            else:
                from_cell.steps = min(from_cell.steps, to_cell.steps + 1)

            queue.append(from_pos)

elmap, start, end = load_input(sys.argv[1])
gen_shortest_paths(elmap, end)
print("Part 1: {}".format(elmap.get(start).steps))
