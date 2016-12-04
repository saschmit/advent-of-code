#!/usr/bin/python

import sys

n = 0
for line in sys.stdin.readlines():
    nums = map(int, line.strip().split())
    nums.sort()
    if nums[0] + nums[1] > nums[2]:
        n += 1

print n
    
