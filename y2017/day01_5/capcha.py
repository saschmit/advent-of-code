#!/usr/bin/python3

import sys

for line in sys.stdin.readlines():
    line = line.strip()

    accum = 0
    i = 0
    ll = len(line)
    interval = int(ll/2)
    while i < ll:
        if line[i] == line[(i+interval) % ll]:
            accum += int(line[i])
        i += 1

    print("%d" % accum)
