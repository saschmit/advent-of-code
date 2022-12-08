#!/usr/bin/python

import sys

for line in sys.stdin.readlines():
    data = line.strip()

    out = ''
    skip = 0
    for i in xrange(len(data)):
        if skip:
            skip -= 1
            continue

        if data[i] != '(':
            out += data[i]
            continue

        x = data.find('x', i+1)
        cl = data.find(')', x+1)
        count = int(data[i+1:x])
        repeat = int(data[x+1:cl])

        out += data[cl+1:cl+1+count] * repeat
        skip = count + cl - i

    print len(out)
