#!/usr/bin/python

import sys

blocks = []

class Block:
    def __init__(self, line):
        self.start, self.end = map(int, line.strip().split('-'))
    def __contains__(self, ip):
        return ip >= self.start and ip <= self.end
    def __str__(self):
        return "[%d, %d]" % (self.start, self.end)
    def __lt__(self, other):
        return self.start < other.start or self.start == other.start and self.end < other.end

for line in sys.stdin.readlines():
    blocks.append(Block(line))

blocks.sort()

for ip in xrange(2**32):
    blocked = False
    if ip % 2**16 == 0:
        print ip

    for block in blocks:
        if ip in block:
            blocked = True
            break
    if not blocked:
        print "unblocked: %d" % ip
        exit(0) 

exit(1)
