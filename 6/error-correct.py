#!/usr/bin/python

import sys

lines = []

for line in sys.stdin.readlines():
    line = line.strip()
    lines.append(line)

output = ""
for c in xrange(len(line)):
    freq = {}
    for r in xrange(len(lines)):
        ch = lines[r][c]
        if ch not in freq:
            freq[ch] = 1
        else:
            freq[ch] += 1

    max_count = 0
    common = ""
    for ch in freq:
        if freq[ch] > max_count:
            max_count = freq[ch]
            common = ch

    output += common

print output
