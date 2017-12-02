#!/usr/bin/python

import sys

spreadsheet = []
for line in sys.stdin.readlines():
    spreadsheet.append(list(map(int, line.split())))

result = 0
for line in spreadsheet:
    line.sort()
    line.reverse()
    answer = None
    for i in range(len(line)):
        for j in range(i+1, len(line)):
            I = line[i]
            J = line[j]
            if I % J == 0:
                answer = int(I / J)
                happy = True
                break
        if answer is not None:
            break
    assert answer is not None
    result += answer

print(result)
