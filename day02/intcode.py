#!/usr/bin/python

from __future__ import division
import sys

def load_pgm(filename):
    return map(int, open(filename).read().strip().split(','))

POSITION_MODE = 0
IMMEDIATE_MODE = 1

def run_pgm(program, get_input=None, put_output=None):
    ip = 0

    def param(param_num, write_val=None):
        param_mode = program[ip] // 10**(2+param_num) % 10 
        if param_mode == POSITION_MODE:
            offset = program[ip + 1 + param_num]
            if write_val is None:
                return program[offset]
            else:
                program[offset] = write_val
        elif param_mode == IMMEDIATE_MODE:
            assert write_val is None
            return program[ip + 1 + param_num]
        else:
            raise NotImplementedError("Illegal parameter mode")


    while True:
        opcode = program[ip] % 100
        if opcode == 1:
            param(2, param(0) + param(1))
            ip += 4
        elif opcode == 2:
            param(2, param(0) * param(1))
            ip += 4
        elif opcode == 3:
            param(0, get_input())
            ip += 2
        elif opcode == 4:
            put_output(param(0))
            ip += 2
        elif opcode == 99:
            ip += 1
            break
        else:
            raise NotImplementedError("Illegal opcode")

    return program

if __name__ == '__main__':
    print(run_pgm(load_pgm(sys.argv[1])))

