#!/usr/bin/python

import sys

lights = [[0 for i in xrange(1000)] for j in xrange(1000)]

def on(bank, x, y):
    bank[x][y] += 1

def off(bank, x, y):
    bank[x][y] = max(0, bank[x][y] - 1)

def flip(bank, x, y):
    bank[x][y] += 2

def mod(bank, start, end, action):
    for x in xrange(start[0], end[0]+1):
        for y in xrange(start[1], end[1]+1):
            action(bank, x, y)

for line in sys.stdin.readlines():
    line = line.strip()
    tokens = line.split(' ')
    if tokens[-2] != 'through':
        raise ValueError
    start = map(int, tokens[-3].split(','))
    end = map(int, tokens[-1].split(','))
    action = None
    if tokens[0] == 'turn':
        if tokens[1] == 'on':
            action = on
        elif tokens[1] == 'off':
            action = off
        else:
            raise ValueError
    elif tokens[0] == 'toggle':
        action = flip
    else:
        raise ValueError

    mod(lights, start, end, action)

count = 0
for x in xrange(0, len(lights)):
    for y in xrange(0, len(lights[x])):
        count += lights[x][y]

print count
