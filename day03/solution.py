#!/usr/bin/python

import sys

rucksacks = []
for line in open(sys.argv[1]):
    line = line.strip()
    mid = len(line) // 2
    rucksacks.append((line[:mid], line[mid:]))
    assert len(rucksacks[-1][0]) == len(rucksacks[-1][1])


#from pprint import pprint
#pprint(rucksacks)

def decode_priority(l):
    if ord('a') <= ord(l) <= ord('z'):
        return ord(l) - ord('a') + 1
    elif ord('A') <= ord(l) <= ord('Z'):
        return ord(l) - ord('A') + 27

part1_sum = 0
for part1, part2 in rucksacks:
    common = set(list(part1)) & set(list(part2))
    part1_sum += decode_priority(common.pop())

print(part1_sum)
