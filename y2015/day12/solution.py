#!/usr/bin/python

import sys
import json

def total_it(j):
    if type(j) is type(0):
        return j

    if type(j) is type({}):
        if "red" not in j.values():
            return sum(map(total_it, j.values()))

    if type(j) is type([]):
        return sum(map(total_it, j))

    return 0

for line in sys.stdin.readlines():
    j = json.loads(line)
    print total_it(j)
