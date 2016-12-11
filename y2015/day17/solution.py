#!/usr/bin/python

import sys

containers = []

liters = int(sys.argv[1])

for line in sys.stdin.readlines():
    containers.append(int(line.strip()))

total = 0
for i in xrange(2**len(containers)):
    used = []
    for j in xrange(len(containers)):
        if i & (1 << j):
            used.append(containers[j])
    if sum(used) == liters:
        total += 1

print total
