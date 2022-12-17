#!/usr/bin/python

import sys

def compare_packets(left, right):
    if type(left) is not type(right):
        if type(left) is type(0):
            #print("Mixed types; convert left and retry")
            left = [left]
        if type(right) is type(0):
            #print("Mixed types; convert right and retry")
            right = [right]

    if type(left) is type(0):
        #print("Compare {} vs {}".format(left, right))
        return left - right
    elif type(left) is type([]):
        #print("Compare {} vs {}".format(left, right))
        shorter = min(len(left), len(right))
        for i in range(shorter):
            res = compare_packets(left[i], right[i])
            if res:
                return res
        #print("Ran out, so compare {} vs {}".format(len(left), len(right)))
        return len(left) - len(right)

class PacketPair:
    def __init__(self, pair):
        self.left = eval(pair[0])
        self.right = eval(pair[1])
    def in_order(self):
        return compare_packets(self.left, self.right) < 0

packet_pairs = []
raw_pairs = open(sys.argv[1]).read().split('\n\n')
for raw_pair in raw_pairs:
    packet_pairs.append(PacketPair(raw_pair.split('\n')))

from pprint import pprint
n = 0
part1_sum = 0
for p in packet_pairs:
    n += 1
    #print("Pair {}".format(n))
    if p.in_order():
        part1_sum += n
    #print(p.in_order())
    #print()

print("Part 1: {}".format(part1_sum))

packets = [
    [[2]],
    [[6]],
]
for p in packet_pairs:
    packets.append(p.left)
    packets.append(p.right)

from functools import cmp_to_key
packets.sort(key=cmp_to_key(compare_packets))
print("Part 2: {}".format((1+packets.index([[2]])) * (1+packets.index([[6]]))))
