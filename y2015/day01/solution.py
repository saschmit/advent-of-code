#!/usr/bin/python

import sys

floor = 0
pos = 0
for l in sys.stdin.read():
    if l == "(":
        floor += 1
        pos += 1
    elif l == ")":
        floor -= 1
        pos += 1
    else:
        continue

    if floor == -1:
        print "1st: ", pos
        

print floor
