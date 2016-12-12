#!/usr/bin/python

import sys

program = []
for line in sys.stdin.readlines():
    tokens = line.strip().split(' ')
    program.append(tokens)

regs = { 'a': 0, 'b': 0, 'c': int(sys.argv[1]), 'd': 0 }

i = 0
while True:
    try:
        tokens = program[i]
    except IndexError:
        break

    if tokens[0] == 'cpy':
        try:
            inp = int(tokens[1])
            regs[tokens[2]] = inp
        except ValueError:
            regs[tokens[2]] = regs[tokens[1]]
    elif tokens[0] == 'inc':
        regs[tokens[1]] += 1
    elif tokens[0] == 'dec':
        regs[tokens[1]] -= 1
    elif tokens[0] == 'jnz':
        val = None
        try:
            val = int(tokens[1])
        except ValueError:
            val = regs[tokens[1]]
        if val:
            i += int(tokens[2])
            continue
    else:
        raise ValueError

    i += 1

print regs['a']
