#!/usr/bin/python

import sys

def isABA(s):
    if '[' in s or ']' in s:
        return False
    return len(s) == 3 and s[0] == s[2] and s[0] != s[1]

def supportsSSL(ip):
    hypernet = False
    abas = set()
    babs = set()
    result = False
    for i in xrange(len(ip)-2):
        triple = ip[i:i+3]
        invtriple = ip[i+1:i+3] + ip[i+1]
        if not hypernet and isABA(triple):
            if triple not in abas:
                if invtriple in babs:
                    return True
                abas.add(triple)
        if hypernet and isABA(triple):
            if triple not in babs:
                if invtriple in abas:
                    return True
                babs.add(triple)
        if ip[i] == '[':
            hypernet = True
        elif ip[i] == ']':
            hypernet = False
    return False

data = []
for line in sys.stdin.readlines():
    line = line.strip()
    data.append(line)

count = 0
for ip in data:
    if supportsSSL(ip):
        count += 1

print count
