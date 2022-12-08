#!/usr/bin/python

import sys

data = {}
deps = {}

class Gate:
    def __init__(self, kind, inputs, output):
        self.__kind = kind
        self.__inputs = inputs
        self.__deps = set(inputs)
        self.__output = output

        for dep in self.__deps:
            if dep not in deps:
                deps[dep] = set()
            deps[dep].add(self)

        for wire in self.__inputs:
            if wire not in data:
                data[wire] = None
                try:
                    data[wire] = int(wire)
                except ValueError:
                    pass

            if data[wire] is not None:
                self.check(wire)
    def check(self, var):
        self.__deps.remove(var)
        deps[var].remove(self)
        if not self.__deps:
            self.__operate()
            if self.__output in deps:
                for gate in deps[self.__output].copy():
                    gate.check(self.__output)
    def __operate(self):
        if self.__kind == 'AND':
            x, y = self.__inputs
            data[self.__output] = (data[x] & data[y]) % 2**16
        elif self.__kind == 'OR':
            x, y = self.__inputs
            data[self.__output] = (data[x] | data[y]) % 2**16
        elif self.__kind == 'NOT':
            assert len(self.__inputs) == 1
            data[self.__output] = (~data[self.__inputs[0]]) % 2**16
        elif self.__kind == 'LSHIFT':
            x, n = self.__inputs
            data[self.__output] = (data[x] << data[n]) % 2**16
        elif self.__kind == 'RSHIFT':
            x, n = self.__inputs
            data[self.__output] = data[x] >> data[n]
        elif self.__kind == 'SET':
            assert len(self.__inputs) == 1
            data[self.__output] = data[self.__inputs[0]]
        else:
            raise ValueError("Unknown operation: %s" % self.__kind)

gates = []

for line in sys.stdin.readlines():
    tokens = line.strip().split(' ')
    assert tokens[-2] == "->"

    if tokens[1] == '->':
        gates.append(Gate('SET', [ tokens[0] ], tokens[2]))
    elif tokens[0] == 'NOT':
        gates.append(Gate('NOT', [ tokens[1] ], tokens[3]))
    else:
        gates.append(Gate(tokens[1], [ tokens[0], tokens[2] ], tokens[4]))

from pprint import pprint
pprint(data)

if 'a' in data:
    print data['a']
