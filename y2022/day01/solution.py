#!/usr/bin/python

import sys

elves = [[]]

for line in open(sys.argv[1]).readlines():
    line = line.strip()
    if line:
        n = int(line)
        elves[-1].append(n)
    else:
        elves.append([])

sum_elves = [sum(elf) for elf in elves]
sum_elves.sort()
print("Part 1: {}".format(sum_elves[-1]))
print("Part 2: {}".format(sum(sum_elves[-3:])))
