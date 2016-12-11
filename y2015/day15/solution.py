#!/usr/bin/python

import sys

class Ingredient:
    def __init__(self, desc):
        self.name, props = desc.split(':')
        props = props.strip().split(', ')
        self.props = {}
        for prop, value in map(lambda p: p.split(' '), props):
            self.props[prop] = int(value)
    def __str__(self):
        s = "%s:" % self.name
        for prop in sorted(self.props):
            s += " %s=%d" % (prop, self.props[prop])
        return s

larder = []
for line in sys.stdin.readlines():
    line = line.strip()
    larder.append(Ingredient(line))

from pprint import pprint
pprint(map(str, larder))

accum = [ 0 ] * len(larder)

def fact(m, n):
    assert m > 0
    if n == 1 or m == 1:
        return n
    return n * fact(m-1, n-1)

def incr(l):
    carry = 1
    for i in xrange(len(l)):
        carry, l[i] = divmod(l[i] + carry, 100)
    assert carry == 0

max_score = 0
for n in xrange(fact(len(larder), 100) + 1):
    incr(accum)
    if sum(accum) != 100:
        continue
    #print accum
    score = 1
    for p in ( 'capacity', 'durability', 'flavor', 'texture'):
        subscore = []
        for i in xrange(len(larder)):
            subscore.append(accum[i] * larder[i].props[p])
        score *= max(0, sum(subscore))
    #print score, max_score
        
    if score > max_score:
        max_score = score

print max_score
