#!/usr/bin/python

import sys

containers = []

liters = int(sys.argv[1])

for line in sys.stdin.readlines():
    containers.append(int(line.strip()))

combinations = []
for i in xrange(2**len(containers)):
    used = []
    for j in xrange(len(containers)):
        if i & (1 << j):
            used.append(containers[j])
    if sum(used) == liters:
        combinations.append(used)

print len(combinations)

least = min(map(len, combinations))

print least

print len(filter(lambda l: len(l) == least, combinations))
