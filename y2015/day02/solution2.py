#!/usr/bin/python

import sys

total = 0
for line in sys.stdin.readlines():
    line = line.strip()
    sides = map(int, line.split('x'))
    sides.sort()
    total += sides[0] * 2 + sides[1] * 2
    total += sides[0] * sides[1] * sides[2]

print total
