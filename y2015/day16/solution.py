#!/usr/bin/python

import sys

analysis = {
    'children': 3,
    'cats': 7,
    'samoyeds': 2,
    'pomeranians': 3,
    'akitas': 0,
    'vizslas': 0,
    'goldfish': 5,
    'trees': 3,
    'cars': 2,
    'perfumes': 1,
}

sues = {}

for line in sys.stdin.readlines():
    line = line.strip()
    sue, props = line.split(':', 1)
    props = map(lambda s: s.strip().split(':'), props.split(','))
    sues[sue] = {}
    for prop, amt in props:
        sues[sue][prop] = int(amt)

for sue in sues:
    match = True
    for prop in sues[sue]:
        if prop in ('cats', 'trees'):
            match = match and (sues[sue][prop] > analysis[prop])
        elif prop in ('pomeranians', 'goldfish'):
            match = match and (sues[sue][prop] < analysis[prop])
        else:
            match = match and (sues[sue][prop] == analysis[prop])
    if match:
        print sue
