#!/usr/bin/python

from hashlib import md5

def crack(key, count=5):
    print "cracking key with %s" % key
    n = 0
    password = ["_"] * 8
    while "_" in password:
        digest = md5(key + str(n)).hexdigest()
        if digest[0:count] == "0" * count:
            pos = digest[count]
            if pos in "01234567":
                pos = int(pos)
                if password[pos] == '_':
                    password[pos] = digest[count+1]
                    print "".join(password)
        n += 1

    return "".join(password)

assert crack("abc") == "05ace8e3"

print crack("wtnhxymk")
