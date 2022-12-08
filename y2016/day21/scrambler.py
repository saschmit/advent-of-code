#!/usr/bin/python

import sys

password = list(sys.argv[1])

def rotl(s, x):
    return s[x:] + s[:x]

def rotr(s, x):
    return s[-x:] + s[:-x]

print "Password: %s" % "".join(password)
for cmd in map(lambda s: s.strip(), sys.stdin.readlines()):
    tokens = cmd.split(' ')
    if tokens[0] == 'swap':
        if tokens[1] == 'position':
            X = int(tokens[2])
            Y = int(tokens[5])
            t = password[X]
            password[X] = password[Y]
            password[Y] = t
            print "swap(%d, %d) -> %s" % (X, Y, "".join(password))
        elif tokens[1] == 'letter':
            X = password.index(tokens[2])
            Y = password.index(tokens[5])
            t = password[X]
            password[X] = password[Y]
            password[Y] = t
            print "swap(%d, %d) -> %s" % (X, Y, "".join(password))
        else:
            raise ValueError("invalid input")
    elif tokens[0] == 'rotate':
        if tokens[1] == 'based':
            X = password.index(tokens[6])
            password = rotr(password, 1)
            password = rotr(password, X)
            if X >= 4:
                password = rotr(password, 1)
            print "rotb(%s) = rotr(%d) -> %s" % (tokens[6], X, "".join(password))
        elif tokens[1] == 'left':
            X = int(tokens[2])
            password = rotl(password, X)
            print "rotl(%d) -> %s" % (X, "".join(password))
        elif tokens[1] == 'right':
            X = int(tokens[2])
            password = rotr(password, X)
            print "rotr(%d) -> %s" % (X, "".join(password))
        else:
            raise ValueError("invalid input")
    elif tokens[0] == 'reverse':
        X = int(tokens[2])
        Y = int(tokens[4])
        password[X:Y+1] = reversed(password[X:Y+1])
        print "reverse(%d, %d) -> %s" % (X, Y, "".join(password))
    elif tokens[0] == 'move':
        X = int(tokens[2])
        Y = int(tokens[5])
        t = password.pop(X)
        password.insert(Y, t)
        print "move(%d, %d) -> %s" % (X, Y, "".join(password))
    else:
        raise ValueError("invalid input")

print "".join(password)
