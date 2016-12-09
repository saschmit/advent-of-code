#!/usr/bin/python

import sys

def num(s, c):
    return len(filter(lambda ch: ch == c, s))

total = 0
for line in sys.stdin.readlines():
    line = line.strip()

    print "%s to %s" % (len(line), len(line) + 2 + num(line, '"') + num(line, '\\'))

    total += 2 + num(line, '"') + num(line, '\\')

print total
