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

def print_map(state):
    trail = state.trail
    current = state.pos
    X = max(map(lambda p: p[0], trail + [current]))
    Y = max(map(lambda p: p[1], trail + [current]))

    print "    " + "0123456789" * ((X+2)//10) + "0123456789"[:(X+2) % 10]
    for y in xrange(Y+2):
        l = "%2d: " % y
        for x in xrange(X+2):
            if iswall(x, y):
                l += "#"
            elif (x, y) == current:
                l += "X"
            elif (x, y) in trail:
                l += "O"
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
    candidates.sort(lambda a, b: cmp(dist(a.pos, target), dist(b.pos, target)) or cmp(len(a.trail), len(b.trail)))
    #print "Candidates:"
    #pprint(candidates)
    #print
    #print "Candidates distances: %s" % map(lambda q: dist(q, target), candidates)

    current = candidates.pop(0)

    print "currently at %s" % current
    #print_map(current)

    if current.pos == target:
        print "found the target! len = %d" % len(current.trail)
        winners.append(current)
        continue

    if current.pos in all_pos:
        print "I've been here before!"
        if len(all_pos[current.pos].trail) > len(current.trail):
            print "But this way is shorter"
            all_pos[current.pos] = current
    else:
        print "Breaking new ground"
        all_pos[current.pos] = current

    for offset in ((0, -1), (0, 1), (1, 0), (-1, 0)):
        trail = current.trail + [ current.pos ]
        cand = (current.x + offset[0], current.y + offset[1])
        print "trying out %s" % (cand,)
        if iswall(cand[0], cand[1]):
            print "NO: %s is a wall" % (cand,)
            continue
        elif cand in all_pos and len(all_pos[cand].trail) <= len(current.trail):
            print "NO: %s; %d has been seen before (or via a longer route)" % (cand, len(all_pos[cand].trail))
            continue
        else:
            print "MAYBE: %s queued" % (cand,)
            candidates.append(State(cand, trail))

shortest_winner = min(map(lambda x: len(x.trail), winners))
for winner in winners:
    if len(winner.trail) == shortest_winner:
        print len(winner.trail)
        print_map(winner)
