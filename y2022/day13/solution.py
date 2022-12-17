#!/usr/bin/python

import sys

class PacketPair:
    def __init__(self, pair):
        self.left = eval(pair[0])
        self.right = eval(pair[1])
    def in_order(self):
        def compare(left, right):
            if type(left) is not type(right):
                if type(left) is type(0):
                    #print("Mixed types; convert left and retry")
                    left = [left]
                if type(right) is type(0):
                    #print("Mixed types; convert right and retry")
                    right = [right]

            if type(left) is type(0):
                #print("Compare {} vs {}".format(left, right))
                return None if left == right else left < right
            elif type(left) is type([]):
                #print("Compare {} vs {}".format(left, right))
                shorter = min(len(left), len(right))
                for i in range(shorter):
                    res = compare(left[i], right[i])
                    if res is not None:
                        return res
                #print("Ran out, so compare {} vs {}".format(len(left), len(right)))
                return None if len(left) == len(right) else len(left) < len(right)
        return compare(self.left, self.right)

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
