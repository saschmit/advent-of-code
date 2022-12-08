#!/usr/bin/python

import sys

def search(paths, so_far, to_go):
    found = False
    for stop in to_go:
        if stop in so_far:
            continue
        found = True
        search(paths, so_far + [stop], m[stop])
    if not found:
        paths.append(so_far)

def compute_max_happiness(paths):
    max_len = 0
    for path in paths:
        path_len = 0 
        for i in xrange(0, len(path)):
            path_len += m[path[i-1]][path[i]] + m[path[i]][path[i-1]]
        max_len = max(path_len, max_len)
    return max_len

m = {}
for line in sys.stdin.readlines():
    line = line.strip()

    tokens = line.split(' ')
    src = tokens[0]
    amt = {"gain": 1, "lose": -1}[tokens[2]] * int(tokens[3])
    dst = tokens[-1][:-1]

    if src not in m:
        m[src] = {}
    m[src][dst] = amt

print "%s guests" % len(m.keys())

seatings = []
search(seatings, [], m.keys())
happiness = compute_max_happiness(seatings)
print happiness

me = 'me'
assert me not in m
m[me] = {}
for guest in m:
    m[me][guest] = 0
    m[guest][me] = 0

print "%s guests & myself" % len(m.keys())

seatings = []
search(seatings, [], m.keys())
happiness = compute_max_happiness(seatings)
print happiness
