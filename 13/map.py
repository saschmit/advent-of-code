#!/usr/bin/python

import sys
from pprint import pprint

favorite = 1364
#favorite = 10

def iswall(x, y, fave=favorite):
    if x < 0 or y < 0:
        return True

    n = x*x + 3*x + 2*x*y + y + y*y + fave
    cursor = 1
    count = 0
    # count bits in n
    while cursor <= n:
        count += int(bool(cursor & n))
        cursor = cursor << 1
    return count % 2

def dist(a, b):
    ax, ay = a
    bx, by = b

    return (ax - bx)**2 + (ay - by)**2

def print_map(all_pos):
    if not all_pos:
        return
    X = max(map(lambda p: p[0], all_pos.keys()))
    Y = max(map(lambda p: p[1], all_pos.keys()))

    print "    " + "0123456789" * ((X+2)//10) + "0123456789"[:(X+2) % 10]
    for y in xrange(Y+2):
        l = "%2d: " % y
        for x in xrange(X+2):
            if iswall(x, y):
                l += "#"
            elif (x, y) in all_pos:
                l += "X"
            else:
                l += "."
        print l
    print

class State:
    def __init__(self, pos, trail):
        self.pos = tuple(pos)
        self.trail = trail

        self.x, self.y = self.pos

    def __contains__(self, d):
        return self.pos in d

    def __str__(self):
        return "(%d, %d; %d)" % (self.x, self.y, len(self.trail))
    __repr__ = __str__

target = (31, 39)
#target = (7, 4)

all_pos = {}

trail = [ ]

candidates = [ State((1,1), []) ]

current = candidates[0]

winners = []

while candidates:
    current = candidates.pop(0)

    print "currently at %s" % current
    print_map(all_pos)

    #if current.pos == target:
    #    print "found the target! len = %d" % len(current.trail)
    #    winners.append(current)
    #    continue
    if len(current.trail) > 50:
        continue

    if current.pos in all_pos:
        if len(all_pos[current.pos].trail) > len(current.trail):
            all_pos[current.pos] = current
    else:
        all_pos[current.pos] = current

    for offset in ((0, -1), (0, 1), (1, 0), (-1, 0)):
        trail = current.trail + [ current.pos ]
        cand = (current.x + offset[0], current.y + offset[1])
        print "trying out %s" % (cand,)
        if iswall(cand[0], cand[1]):
            print "NO: %s is a wall" % (cand,)
            continue
        elif cand in all_pos:
            print "NO: %s has been seen before" % (cand,)
            continue
        else:
            print "MAYBE: %s queued" % (cand,)
            candidates.append(State(cand, trail))

print(len(all_pos))

print(all_pos[(16,23)])
