#!/usr/bin/python

import sys

favorite = 1364
#favorite = 10

def iswall(x, y, fave=favorite):
    if x < 0 or y < 0:
        return True

    n = x*x + 3*x + 2*x*y + y + y*y + fave
    cursor = 1
    count = 0
    while cursor <= n:
        count += int(bool(cursor & n))
        cursor = cursor << 1
    return count % 2

def dist(a, b):
    ax, ay = a
    bx, by = b

    return (ax - bx)**2 + (ay - by)**2

def print_map(trail, current):
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

target = (31, 39)
#target = (7, 4)

current = (1, 1)

all_pos = {}

trail = [ ]

print current
print_map(trail, current)
while current != target:
    candidates = []
    x, y = current
    if current in all_pos:
        print "I've been here before!"
    else:
        print "Breaking new ground"

    for offset in ((0, -1), (0, 1), (1, 0), (-1, 0)):
        cand = (current[0] + offset[0], current[1] + offset[1])
        print "trying out %s" % (cand,)
        if iswall(cand[0], cand[1]):
            print "NO: %s is a wall" % (cand,)
            continue
        elif cand in all_pos:
            print "NO: %s has been seen before" % (cand,)
            continue
        else:
            print "MAYBE: %s queued" % (cand,)
            candidates.append(cand)

    if candidates:
        candidates.sort(lambda a, b: cmp(dist(a, target), dist(b, target)))
        print "Candidates found: %s" % candidates
        #print "Candidates distances: %s" % map(lambda q: dist(q, target), candidates)

        nxt = candidates.pop(0)
        all_pos[current] = candidates
        trail.append(current)
        current = nxt
    else:
        all_pos[current] = candidates
        current = trail.pop()
        print "Backtracking to %s" % (current, )

    print current
    print_map(trail, current)

print len(trail)
