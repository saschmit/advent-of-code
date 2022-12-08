#!/usr/bin/python3

import sys

for line in sys.stdin.readlines():
    line = line.strip()

    accum = 0
    i = 0
    ll = len(line)
    while i < ll:
        if line[i] == line[(i+1) % ll]:
            accum += int(line[i])
        i += 1

    print("%d" % accum)
