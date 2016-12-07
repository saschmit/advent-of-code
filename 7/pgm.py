#!/usr/bin/python

import sys

def isABBA(s):
    if '[' in s or ']' in s:
        return False
    return len(s) == 4 and s[0] == s[3] and s[1] == s[2] and s[0] != s[1]

def isABA(s):
    if '[' in s or ']' in s:
        return False
    return len(s) == 3 and s[0] == s[2] and s[0] != s[1]

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

tls_count = 0
ssl_count = 0
for ip in data:
    if supportsTLS(ip):
        tls_count += 1
    if supportsSSL(ip):
        ssl_count += 1

print "TLS: %s" % tls_count
print "SSL: %s" % ssl_count
