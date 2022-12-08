#!/usr/bin/python

import sys

def isTri(nums):
    nums.sort()
    if nums[0] + nums[1] > nums[2]:
        return True
    return False

grid = []
for line in sys.stdin.readlines():
    nums = map(int, line.strip().split())
    grid.append(nums)

ctgrid = []
for i in xrange(0, len(grid), 3):
    ctgrid.append([grid[i][0], grid[i+1][0], grid[i+2][0]])
    ctgrid.append([grid[i][1], grid[i+1][1], grid[i+2][1]])
    ctgrid.append([grid[i][2], grid[i+1][2], grid[i+2][2]])

n = 0
for line in ctgrid:
    nums = line[:]
    if isTri(nums):
        n += 1

print n
    
