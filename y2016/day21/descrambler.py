#!/usr/bin/python

import sys

password = list(sys.argv[1])

def rotl(s, x):
    return s[x:] + s[:x]

def rotr(s, x):
    return s[-x:] + s[:-x]

def rotb(s, c):
    x = s.index(c)
    s = rotr(s, 1)
    s = rotr(s, x)
    if x >= 4:
        s = rotr(s, 1)
    return s

print "Password: %s" % "".join(password)
cmds = []
for cmd in map(lambda s: s.strip(), sys.stdin.readlines()):
    cmds.append(cmd)

cmds.reverse()

for cmd in cmds:
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
            cand = password
            orig = password
            while rotb(cand, tokens[6]) != orig:
                cand = rotl(cand, 1)
            password = cand
            print "rotb(%s) -> %s" % (tokens[6], "".join(password))
        elif tokens[1] == 'left':
            X = int(tokens[2])
            password = rotr(password, X)
            print "rotl(%d) -> %s" % (X, "".join(password))
        elif tokens[1] == 'right':
            X = int(tokens[2])
            password = rotl(password, X)
            print "rotr(%d) -> %s" % (X, "".join(password))
        else:
            raise ValueError("invalid input")
    elif tokens[0] == 'reverse':
        X = int(tokens[2])
        Y = int(tokens[4])
        password[X:Y+1] = reversed(password[X:Y+1])
        print "reverse(%d, %d) -> %s" % (X, Y, "".join(password))
    elif tokens[0] == 'move':
        Y = int(tokens[2])
        X = int(tokens[5])
        t = password.pop(X)
        password.insert(Y, t)
        print "move(%d, %d) -> %s" % (X, Y, "".join(password))
    else:
        raise ValueError("invalid input")

print "".join(password)
