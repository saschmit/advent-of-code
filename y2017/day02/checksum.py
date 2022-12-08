#!/usr/bin/python

import sys

spreadsheet = []
for line in sys.stdin.readlines():
    spreadsheet.append(list(map(int, line.split())))

checksum = 0
for line in spreadsheet:
    checksum += max(line) - min(line)

print checksum
