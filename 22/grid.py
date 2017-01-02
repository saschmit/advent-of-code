#!/usr/bin/python

import sys

grid = {}

sys.stdin.readline()
sys.stdin.readline()
maxX = -1
maxY = -1
for line in map(lambda s: s.strip(), sys.stdin.readlines()):
    dev, total, used, free, _ = line.split()
    assert total[-1] == 'T'
    assert used[-1] == 'T'
    assert free[-1] == 'T'
    total = int(total[:-1])
    used = int(used[:-1])
    free = int(free[:-1])
    assert total == used + free

    prefix, x, y = dev.split('-')
    assert prefix == '/dev/grid/node'
    x = int(x[1:])
    y = int(y[1:])

    grid[(x,y)] = (total, used, free)
    maxX = max(x, maxX)
    maxY = max(y, maxY)

from pprint import pprint
#pprint(grid)
print maxX, maxY

TOTAL = 0
USED = 1
AVAIL = 2
viables = set()
unwalls = set()
for nodeA in grid:
    for nodeB in grid:
        if grid[nodeA][USED] != 0 and nodeA is not nodeB and grid[nodeA][USED] <= grid[nodeB][AVAIL]:
            viables.add((nodeA,nodeB))
            unwalls.add(nodeA)

print len(viables)

def print_grid(grid, maxX, maxY):
    goal = None
    empty = None
    leftWall = None
    for y in xrange(0, maxY+1):
        line = ""
        for x in xrange(0, maxX+1):
            node = grid[(x,y)]
            if not x and not y:
                line += 'O'
            elif x == maxX and not y:
                line += 'G'
                goal = (x,y)
            elif node[USED] == 0:
                line += '_'
                empty = (x,y)
            elif (x,y) not in unwalls:
                line += '#'
                if leftWall is None:
                    leftWall = (x,y)
                elif leftWall < x:
                    leftWall = (x,y)
            else:
                line += '.'
        print line
    print "Goal: %d, %d" % goal
    print "Empty: %d, %d" % empty
    print "Left wall: %d, %d" % leftWall

print_grid(grid, maxX, maxY)

