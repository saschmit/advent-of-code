#!/usr/bin/python

import sys

def load_pgm(filename):
    return map(int, open(filename).read().strip().split(','))

def run_pgm(program):
    ip = 0
    while True:
        instruction = program[ip]
        if instruction == 1:
            src1 = program[ip + 1]
            src2 = program[ip + 2]
            dst = program[ip + 3]
            program[dst] = program[src1] + program[src2]
        elif instruction == 2:
            src1 = program[ip + 1]
            src2 = program[ip + 2]
            dst = program[ip + 3]
            program[dst] = program[src1] * program[src2]
        elif instruction == 99:
            break
        else:
            raise NotImplementedError("Illegal instruction")

        ip += 4

    return program

if __name__ == '__main__':
    print(run_pgm(load_pgm(sys.argv[1])))

