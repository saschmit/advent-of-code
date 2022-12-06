#!/usr/bin/python

import sys

for signal in open(sys.argv[1]):
    signal = signal.strip()
    for n in range(3, len(signal)):
        candidate = set(list(signal[n-3:n+1]))
        if len(candidate) == 4:
            print(n+1)
            break
