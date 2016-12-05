#!/usr/bin/python

from hashlib import md5

def mine(key, count=5):
    n = 0
    digest = ""
    while digest != "0" * count:
        n += 1
        digest = md5(key + str(n)).hexdigest()[0:count]

    return n

assert mine("abcdef") == 609043
assert mine("pqrstuv") == 1048970

print mine("yzbqklnj")
print mine("yzbqklnj", 6)
