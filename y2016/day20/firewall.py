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

print "raw: ", len(blocks)

old_len = None
new_len = len(blocks)
while old_len != new_len:
    i = 0
    while (i+1) < len(blocks):
        if (blocks[i].end+1) >= blocks[i+1].start:
            if blocks[i].end >= blocks[i+1].end:
                del blocks[i+1]
            else:
                blocks[i] = Block("%d-%d" % (blocks[i].start, blocks[i+1].end))
                del blocks[i+1]
        else:
            i += 1

    old_len = new_len
    new_len = len(blocks)

print "optimized: ", len(blocks)

unblocked = blocks[0].start
print "first unblocked = ", (blocks[0].end + 1)
for i in xrange(1, len(blocks)):
    unblocked += blocks[i].start - blocks[i-1].end - 1

print "total unblocked = ", unblocked
