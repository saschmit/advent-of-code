#!/usr/bin/python

import sys

for line in sys.stdin.readlines():
    accum = ''
    total = 0
    sign = 1
    for c in line:
        if c in '0123456789':
            accum += c
        elif c == '-':
            sign = -1
        elif accum:
            total += sign * int(accum)
            accum = ''
            sign = 1

    print total
