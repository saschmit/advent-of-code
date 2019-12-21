#!/usr/bin/python

def is_valid(candidate):
    if len(candidate) != 6:
        return False

    double = False
    for i in xrange(len(candidate)-1):
        if candidate[i] > candidate[i+1]:
            return False
        elif candidate[i] == candidate[i+1]:
            double = True

    return double

assert is_valid((1,1,1,1,1,1))
assert not is_valid((2,2,3,4,5,0))
assert not is_valid((1,2,3,7,8,9))

lower = 134564
upper = 585159

count = 0
for n in xrange(lower, upper+1):
    candidate = map(int, list(str(n)))
    if is_valid(candidate):
        count += 1

print(count)

