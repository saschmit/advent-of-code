#!/usr/bin/python

import sys

houses = {'0,0': 2 }

instr = sys.stdin.read()

def follow(script, houses):
    x = 0
    y = 0
    for c in script:
        if c == '^':
            y += 1
        elif c == 'v':
            y -= 1
        elif c == '<':
            x -= 1
        elif c == '>':
            x += 1
        else:
            continue

        key = "%s,%s" % (x, y)
        if key not in houses:
            houses[key] = 0

        houses[key] += 1

follow(instr[0::2], houses)
follow(instr[1::2], houses)

print(len(houses.keys()))
