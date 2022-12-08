#!/usr/bin/python

from hashlib import md5
import sys
from pprint import pprint

salt = sys.stdin.read().strip()

hashes = {}
all_triples = {}
keys = []

def genrand(n):
    h = md5(salt + str(n)).hexdigest()
    for i in xrange(2016):
        h = md5(h).hexdigest()
    return h

def iskey(n):
    candidate = None
    if n not in hashes:
        hashes[n] = genrand(n)
    candidate = hashes[n]
    triple = get_triple(candidate)
    if not triple:
        return False
    all_triples[n] = triple
    for i in xrange(n+1, n+1001):
        if i not in hashes:
            hashes[i] = genrand(i)
    if filter(lambda x: triple[0] * 5 in x, map(lambda i: hashes[i], xrange(n+1, n+1001))):
        return True
    return False

def get_triple(candidate):
    for i in xrange(len(candidate)-2):
        if candidate[i] == candidate[i+1] == candidate[i+2]:
            return candidate[i]
    return ""

if False:
    pprint(gen_triples(genrand(18)))
    print iskey(18)
    pprint(gen_triples(genrand(39)))
    print iskey(39)

n = 0
while len(keys) < 64:
    if n % 1000 == 0:
        print n
    if iskey(n):
        keys.append((n, hashes[n]))
    n += 1

pprint(all_triples)
print len(keys)
pprint(keys)
