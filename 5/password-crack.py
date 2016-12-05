#!/usr/bin/python

from hashlib import md5

def mine(key, count=5):
    n = 0
    password = ""
    while len(password) < 8:
        digest = md5(key + str(n)).hexdigest()
        if digest[0:count] == "0" * count:
            password += digest[count]
        n += 1

    return password

assert mine("abc") == "18f47a30"

print mine("wtnhxymk")
