#!/usr/bin/python

def factor(n):
    factors = []
    for i in xrange(1, n+1):
        if n % i == 0:
            factors.append(i)
    print(factors)
    return sum(factors)

print("Part 1 = %d" % factor(926))
print("Part 2 = %d" % factor(10551326))
