#!/usr/bin/python

import sys

def decomp(data):
    l = 0
    skip = 0
    for i in xrange(len(data)):
        if skip:
            skip -= 1
            continue

        if data[i] != '(':
            l += 1
            continue

        x = data.find('x', i+1)
        cl = data.find(')', x+1)
        count = int(data[i+1:x])
        repeat = int(data[x+1:cl])

        l += decomp(data[cl+1:cl+1+count]) * repeat
        skip = count + cl - i
    return l

for line in sys.stdin.readlines():
    data = line.strip()

    print decomp(data)
