#!/usr/bin/python

import sys

class Ingredient:
    def __init__(self, desc):
        self.name, props = desc.split(':')
        props = props.strip().split(', ')
        self.props = {}
        for prop, value in map(lambda p: p.split(' '), props):
            self.props[prop] = int(value)

larder = []
for line in sys.stdin.readlines():
    line = line.strip()
    larder.append(Ingredient(line))

from pprint import pprint
pprint(larder)
