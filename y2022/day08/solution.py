#!/usr/bin/python

import sys
from enum import Enum

grid = []
height = 0
width = 0
for line in open(sys.argv[1]):
    line = line.strip()
    if not width:
        width = len(line)
    grid += [list(line)]
    height += 1

def is_visible(grid, row, col):
    # Degenerate cases
    if row == 0 or col == 0:
        return True
    elif row == height - 1 or col == width - 1:
        return True

    tree_height = grid[row][col]

    # Look left
    clear = True
    for c in range(0, col):
        if grid[row][c] >= tree_height:
            clear = False
    if clear:
        return clear

    # Look right
    clear = True
    for c in range(col+1, width):
        if grid[row][c] >= tree_height:
            clear = False
    if clear:
        return clear

    # Look up
    clear = True
    for r in range(0, row):
        if grid[r][col] >= tree_height:
            clear = False
    if clear:
        return clear

    # Look down
    clear = True
    for r in range(row+1, height):
        if grid[r][col] >= tree_height:
            clear = False
    if clear:
        return clear


    return False

part1_vis = 0
for row in range(height):
    for col in range(width):
        part1_vis += 1 if is_visible(grid, row, col) else 0

print("Part 1: {}".format(part1_vis))
