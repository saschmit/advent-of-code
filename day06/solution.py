#!/usr/bin/python

import sys

for signal in open(sys.argv[1]):
    signal = signal.strip()
    for n in range(3, len(signal)):
        candidate = set(list(signal[n-3:n+1]))
        if len(candidate) == 4:
            print("Part 1: {}".format(n+1))
            break
    for n in range(13, len(signal)):
        candidate = set(list(signal[n-13:n+1]))
        if len(candidate) == 14:
            print("Part 2: {}".format(n+1))
            break
