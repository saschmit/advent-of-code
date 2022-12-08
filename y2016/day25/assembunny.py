#!/usr/bin/python

import sys

program = []
for line in sys.stdin.readlines():
    tokens = line.strip().split(' ')
    program.append(tokens)

regs = { 'a': int(sys.argv[1]), 'b': 0, 'c': 0, 'd': 0 }

i = 0
output = None
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
    elif tokens[0] == 'out':
        old = output
        output = regs[tokens[1]]
        if old is not None:
            if old == output:
                exit(1)
    else:
        raise ValueError

    i += 1

print regs['a']
