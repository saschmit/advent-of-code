#!/usr/bin/python

import sys

total = 0
for line in sys.stdin.readlines():
    line = line.strip()

    raw_len = len(line)

    assert line[0] == line[-1] and line[0] == '"'

    cooked_len = 0
    skip = 0
    for i in xrange(1, len(line)-1):
        if skip:
            skip -= 1
            continue
        cooked_len += 1
        if line[i] == '\\':
            if line[i+1] == '\\':
                skip = 1
            elif line[i+1] == '"':
                skip = 1
            elif line[i+1] == 'x':
                assert line[i+2] in '0123456789abcdefABCDEF'
                assert line[i+3] in '0123456789abcdefABCDEF'
                skip = 3
            else:
                raise ValueError("Unknown escape code: \\%s" % line[i+1])

    print "%d - %d = %d" % (raw_len, cooked_len, raw_len - cooked_len)
    total += raw_len - cooked_len

print total
