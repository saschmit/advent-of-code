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

part2_sum = 0
for group in range(len(rucksacks) // 3):
    rucksack0 = "".join(rucksacks[group * 3])
    common = set(list(rucksack0))
    for n in range(1, 3):
        idx = group * 3 + n
        common &= set(list("".join(rucksacks[idx])))
    part2_sum += decode_priority(common.pop())

print(part2_sum)

