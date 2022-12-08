#!/usr/bin/python

import sys

width, height = map(int, sys.argv[1:3])

screen = [['.' for x in xrange(width)] for y in xrange(height)]

for line in sys.stdin.readlines():
    line = line.strip()

    tokens = line.split(' ')
    if tokens[0] == 'rect':
        w, h = map(int, tokens[1].split('x'))
        for r in xrange(h):
            for c in xrange(w):
                screen[r][c] = '#'
    elif tokens[0] == 'rotate':
        count = int(tokens[4])
        ndx = int(tokens[2].split('=')[1])
        if tokens[1] == 'row':
            v = screen[ndx][:]
            for i in xrange(width):
                screen[ndx][(i + count) % width] = v[i]
        elif tokens[1] == 'column':
            v = [0] * height
            for i in xrange(height):
                v[i] = screen[i][ndx]
            for i in xrange(height):
                screen[(i + count) % height][ndx] = v[i]
        else:
            raise ValueError("Parse error: %s" % tokens[1])
    else:
        raise ValueError("Unknown command: %s" % tokens[0])

on = 0
for r in xrange(height):
    for c in xrange(width):
        if screen[r][c] == '#':
            on += 1

print on

for r in xrange(height):
    print "".join(screen[r])
