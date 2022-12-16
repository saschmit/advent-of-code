#!/usr/bin/python

import sys

pgm = []
for line in open(sys.argv[1]):
    tokens = line.strip().split()
    if tokens[0] == 'noop':
        pgm.append(('noop', None))
    elif tokens[0] == 'addx':
        pgm.append(('addx', int(tokens[1])))
    else:
        assert False

op_cycle_table = {
    'noop': 1,
    'addx': 2,
}

def run(pgm, visitor, state):
    pgm = list(pgm)
    regs = { 'X': 1 }
    cycle = 0
    op = None

    while True:
        cycle += 1

        if op is None:
            op_cycle = 0
            try:
                op, arg = pgm.pop(0)
            except IndexError:
                break

        op_cycle += 1

        state = visitor(state, cycle, regs)

        if op == 'noop':
            pass
        elif op == 'addx':
            if op_cycle == 2:
                regs['X'] += arg

        if op_cycle == op_cycle_table[op]:
            op = None


    return state

def calc_signal_strength(state, cycle, regs):
    #print((cycle, regs['X']))
    if (cycle + 20) % 40 == 0:
        state += cycle * regs['X']

    return state

result = run(pgm, calc_signal_strength, 0)

print("Part 1: {}".format(result))
