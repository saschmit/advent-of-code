#!/usr/bin/python

import sys

class Instruction:
    def __init__(self, tokens):
        assert tokens[0] == 'move'
        assert tokens[2] == 'from'
        assert tokens[4] == 'to'
        self.count = int(tokens[1])
        self.start = int(tokens[3]) - 1
        self.dest = int(tokens[5]) - 1
    def __str__(self):
        return "{} x {} -> {}".format(self.count, self.start, self.dest)

insns = []
stacks = None
stage = 'stacks'
for line in open(sys.argv[1]):
    if stage == 'stacks':
        if '1' not in line:
            assert len(line) % 4 == 0
            if stacks is None:
                stacks = [[] for n in range(len(line) // 4)]

            for i in range(0, len(line), 4):
                n = i // 4
                char = line[i+1:i+2]
                if char != ' ':
                    stacks[n].insert(0, char)
        else:
            stage = 'stacklabels'
    if stage == 'stacklabels':
        if not line.strip():
            stage = 'insns'
            continue
        else:
            continue
    if stage == 'insns':
        tokens = line.strip().split()
        insns.append(Instruction(tokens))

from pprint import pprint

#pprint([str(insn) for insn in insns])

for insn in insns:
    for n in range(insn.count):
        stacks[insn.dest].append(stacks[insn.start].pop())

pprint(stacks)
print("Part 1: {}".format("".join([stack[-1] for stack in stacks])))
