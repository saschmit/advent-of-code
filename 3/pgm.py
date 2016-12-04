#!/usr/bin/python

import sys

def isTri(nums):
    nums.sort()
    if nums[0] + nums[1] > nums[2]:
        return True
    return False

n = 0
for line in sys.stdin.readlines():
    nums = map(int, line.strip().split())
    if isTri(nums):
        n += 1

print n
    
