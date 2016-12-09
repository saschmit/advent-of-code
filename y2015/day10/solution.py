#!/usr/bin/python

def las(seq):
    seq = map(int, seq)
    count = 0
    run = None
    output = ''
    for n in seq:
        if n == run:
            count += 1
        else:
            if count:
                output += str(count) + str(run)
            run = n
            count = 1
    if count:
        output += str(count) + str(run)
    return output

seq = '1'
answers = ['11', '21', '1211', '111221', '312211']
for answer in answers:
    result = las(seq)
    print seq, result, answer, answer == result
    seq = result

assert las(las(las(las(las("1"))))) == "312211"

seq = "3113322113"
for i in xrange(40):
    seq = las(seq)

print len(seq)

for i in xrange(10):
    seq = las(seq)

print len(seq)
