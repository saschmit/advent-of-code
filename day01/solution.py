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

print(max([sum(elf) for elf in elves]))
