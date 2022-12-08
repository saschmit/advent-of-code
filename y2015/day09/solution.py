#!/usr/bin/python

import sys
from pprint import pprint

m = {}
for line in sys.stdin.readlines():
    line = line.strip()

    src, dst, dist = line.split(' ')[0::2]
    dist = int(dist)

    for x, y in ((src, dst), (dst, src)):
        if x not in m:
            m[x] = {}
        m[x][y] = dist

paths = []

def search(so_far, to_go):
    found = False
    for stop in to_go:
        if stop in so_far:
            continue
        found = True
        search(so_far + [stop], m[stop])
    if not found:
        paths.append(so_far)

search([], m.keys())

min_len = None
max_len = None
for path in paths:
    path_len = 0 
    for i in xrange(1, len(path)):
        path_len += m[path[i-1]][path[i]]
    if min_len is None:
        min_len = path_len
    else:
        min_len = min(path_len, min_len)
    max_len = max(path_len, max_len)

print min_len, max_len
