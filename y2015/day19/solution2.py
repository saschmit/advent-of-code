#!/usr/bin/python

import sys

molecule = None
xforms = []
for line in sys.stdin.readlines():
    tokens = line.strip().split(' ')
    if len(tokens) == 1:
        molecule = tokens[0]
    elif len(tokens) == 0:
        continue
    elif len(tokens) == 3 and tokens[1] == '=>':
        xforms.append(tokens[0::2])
    else:
        raise ValueError

print molecule
print xforms

for rule_in, rule_out in xforms:
    n = molecule.find(rule_in)
    while n != -1:
        molecule[:n] + rule_out + molecule[n+len(rule_in):]
        n = molecule.find(rule_in, n+1)

print len(poss)
