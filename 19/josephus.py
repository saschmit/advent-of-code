#!/usr/bin/python

n = 3001330
#n = 5

import sys
import math

class Node:
    def __init__(self, val):
        self.val = val
        self.nxt = None

head = Node(1)
tail = head
for i in xrange(2, n+1):
    new = Node(tail.val+1)
    tail.nxt = new
    tail = new

assert tail.val == n

tail.nxt = head
count = n

cursor = head
while count != 1:
    kill = cursor
    for i in xrange(count//2):
        kill = head.nxt

        kill.nxt = kill.nxt.nxt
    cursor = cursor.nxt
    count -= 1
    if count % 10 == 0:
        print count

print cursor.val
exit(0)

if True:
    def find_next(total, offset):
        return ((total // 2) + offset) % total

    for n in xrange(1, 101):
        elves = range(1, n+1)

        #print elves
        count = 0
        while len(elves) != 1:
            if len(elves) % 10 == 0:
                print len(elves)
            new_count = (count + 1) % len(elves)
            del elves[find_next(len(elves), count)]
            count = new_count
            #print elves, count

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
