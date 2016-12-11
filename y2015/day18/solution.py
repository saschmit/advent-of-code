#!/usr/bin/python

import sys

ON = '#'
OFF = '.'

steps = int(sys.argv[1])

lights = []
for line in sys.stdin.readlines():
    line = line.strip()
    lights.append(line)

def count_on(grid, i, j):
    rows, cols = len(grid), len(grid[0])
    on = 0
    for r in xrange(max(0, i-1), min(rows, i+2)):
        for c in xrange(max(0, j-1), min(cols, j+2)):
            if grid[r][c] == ON and not ((r - i) == 0 and (c - j) == 0):
                on += 1
    return on

def live(grid):
    output = []
    for i in xrange(len(grid)):
        line = ""
        for j in xrange(len(grid[i])):
            state = None
            if grid[i][j] == ON:
                if count_on(grid, i, j) in (2, 3):
                    state = ON
                else:
                    state = OFF
            elif grid[i][j] == OFF:
                if count_on(grid, i, j) == 3:
                    state = ON
                else:
                    state = OFF
            line += state
        output.append(line)
    return output

def print_lights(grid):
    for i in xrange(len(grid)):
        print grid[i]
    print

def count_lights(grid):
    count = 0
    for i in xrange(len(grid)):
        for j in xrange(len(grid[i])):
            if grid[i][j] == ON:
                count += 1
    return count

for i in xrange(steps):
    lights = live(lights)

#print_lights(lights)
print count_lights(lights)
