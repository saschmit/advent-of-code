#!/usr/bin/python

import sys

total = 0
for line in sys.stdin.readlines():
    line = line.strip()
    l, w, h = map(int, line.split('x'))
    sa = 2*l*w + 2*w*h + 2*h*l
    areas = l*w, w*h, h*l
    smallest = sorted(areas)[0]
    assert sa == 2 * sum(areas)
    total += sa + smallest

print total
