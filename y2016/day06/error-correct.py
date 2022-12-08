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

    min_count = len(lines)
    common = ""
    for ch in freq:
        if freq[ch] < min_count:
            min_count = freq[ch]
            common = ch

    output += common

print output
