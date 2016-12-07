#!/usr/bin/python

import sys

def isABBA(s):
    if '[' in s or ']' in s:
        return False
    return len(s) == 4 and s[0] == s[3] and s[1] == s[2] and s[0] != s[1]

def supportsTLS(ip):
    hypernet = False
    result = False
    for i in xrange(len(ip)):
        if hypernet and isABBA(ip[i:i+4]):
            return False
        if not hypernet and isABBA(ip[i:i+4]):
            result = True
        if ip[i] == '[':
            hypernet = True
        elif ip[i] == ']':
            hypernet = False
    return result

data = []
for line in sys.stdin.readlines():
    line = line.strip()
    data.append(line)

count = 0
for ip in data:
    if supportsTLS(ip):
        count += 1

print count
