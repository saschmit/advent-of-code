#!/usr/bin/python

n = 3001330
#n = 5

import sys
import math

if True:
    def find_next(total, offset):
        return ((total // 2) + offset) % total

    elves = range(1, n+1)

    count = 0
    while len(elves) != 1:
        new_count = (count + 1) % len(elves)
        del elves[find_next(len(elves), count)]
        count = new_count

    print n, elves
else:
    import math
    import sys
    n = int(sys.argv[1])
    count = int(math.log(n) / math.log(2))
    least2 = count
    n1 = 1
    n2 = 1
    count -= 2
    while count:
        n1, n2 = (n2, n1+n2)
        count -= 1
    print n, n1, n2, least2
    fib = n2
    print fib + (n - 2**least2)//2
