#!/usr/bin/python

import sys

TRAP = '^'
SAFE = '.'

def is_trap(l, c, r):
    return (l == c and c != r) or (l != c and c == r)

rows = [ sys.stdin.readline().strip() ]

def gen_row(row):
    row = SAFE + row + SAFE
    new = ""
    for i in xrange(1, len(row)-1):
        new += TRAP if is_trap(row[i-1], row[i], row[i+1]) else SAFE
    return new

count = 1
num_rows = int(sys.argv[1])

for i in xrange(count, num_rows):
    rows.append(gen_row(rows[-1]))

def print_rows(rows):
    for row in rows:
        print row

print_rows(rows)
print sum(map(lambda s: s.count(SAFE), rows))
