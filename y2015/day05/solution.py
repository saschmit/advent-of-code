#!/usr/bin/python

import sys

def doubles(kid):
    for i in xrange(2, len(kid)):
        if kid[i] == kid[i-2]:
            #print "'%s' contains '%s'" % (kid, kid[i-2:i+1])
            return True
    return False

def dupe_pairs(kid):
    for i in xrange(0, len(kid)):
        for j in xrange(i+2, len(kid)):
            if kid[i:i+2] == kid[j:j+2]:
                #print "'%s' contains two instances of '%s'" % (kid, kid[i:i+2])
                return True
    return False

def nice(kid):
    return doubles(kid) and dupe_pairs(kid)

assert doubles("xyx")
assert doubles("abcdefeghi")
assert doubles("aaa")

assert dupe_pairs("xyxy")
assert dupe_pairs("aabcdefgaa")
assert not dupe_pairs("aaa")

assert nice("qjhvhtzxzqqjkmpb")
assert nice("xxyxx")
assert not nice("uurcxstgmygtbstg")
assert not nice("ieodomkazucvgmuy")

kids = []
for line in sys.stdin.readlines():
    line = line.strip()

    kids.append(line)

for kid in kids:
    if nice(kid):
        print kid
