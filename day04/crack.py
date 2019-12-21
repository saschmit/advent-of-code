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

def is_valid2(candidate):
    if len(candidate) != 6:
        return False

    repeat = 1
    double = False
    for i in xrange(len(candidate)-1):
        if candidate[i] > candidate[i+1]:
            return False

        if i == 0 and candidate[i] == candidate[i+1]:
            repeat += 1
        elif i > 0:
            if candidate[i] == candidate[i+1]:
                repeat += 1
            elif repeat == 2:
                repeat = 1
                double = True
            else:
                repeat = 1

    if repeat == 2:
        return True

    return double

assert is_valid((1,1,1,1,1,1))
assert not is_valid((2,2,3,4,5,0))
assert not is_valid((1,2,3,7,8,9))

assert is_valid2((1,1,2,2,3,3))
assert not is_valid2((1,2,3,4,4,4))
assert is_valid2((1,1,1,1,2,2))

lower = 134564
upper = 585159

count = 0
count2 = 0
for n in xrange(lower, upper+1):
    candidate = map(int, list(str(n)))
    if is_valid(candidate):
        count += 1
    if is_valid2(candidate):
        count2 += 1

print(count)
print(count2)
