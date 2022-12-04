#!/usr/bin/python

import sys

def pair_contains(p1, p2):
    return p1[0] >= p2[0] and p1[1] <= p2[1]

def pair_overlap(p1, p2):
    return p2[0] <= p1[1] and p2[1] >= p1[1]

count1 = 0
count2 = 0
for line in open(sys.argv[1]):
    pair1, pair2 = line.strip().split(',')
    pair1 = [int(n) for n in pair1.split('-')]
    pair2 = [int(n) for n in pair2.split('-')]
    if pair_contains(pair1, pair2) or pair_contains(pair2, pair1):
        count1 += 1
    if pair_overlap(pair1, pair2) or pair_overlap(pair2, pair1):
        count2 += 1

print(count1)
print(count2)
