#!/usr/bin/python3

import sys

def load_pgm(filename):
    return list(map(int, open(filename).read().strip().split(',')))

POSITION_MODE = 0
IMMEDIATE_MODE = 1
RELATIVE_MODE = 2

class Memory:
    def __init__(self):
        self.__mem = {}
    def __getitem__(self, key):
        if type(key) is not type(0):
            raise ValueError("key is an invalid type")

        if key in self.__mem:
            return self.__mem[key]
        elif key > 0:
            return 0

        return self.__mem[key]
    def __setitem__(self, key, value):
        if type(key) is not type(0):
            raise ValueError("key is an invalid type: {}".format(type(key)))

        if key < 0:
            self.__mem[key]
        else:
            self.__mem[key] = value

def run_pgm(program, get_input=None, put_output=None):
    memory = Memory()
    for offset in range(len(program)):
        memory[offset] = program[offset]
    ip = 0
    base = 0

    def param(param_num, write_val=None):
        param_mode = memory[ip] // 10**(2+param_num) % 10 
        if param_mode == POSITION_MODE:
            offset = memory[ip + 1 + param_num]
            if write_val is None:
                return memory[offset]
            else:
                memory[offset] = write_val
        elif param_mode == IMMEDIATE_MODE:
            assert write_val is None
            return memory[ip + 1 + param_num]
        elif param_mode == RELATIVE_MODE:
            offset = base + memory[ip + 1 + param_num]
            if write_val is None:
                return memory[offset]
            else:
                memory[offset] = write_val
        else:
            raise NotImplementedError("Illegal parameter mode")


    while True:
        opcode = memory[ip] % 100
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
        elif opcode == 5:
            if param(0) != 0:
                ip = param(1)
            else:
                ip += 3
        elif opcode == 6:
            if param(0) == 0:
                ip = param(1)
            else:
                ip += 3
        elif opcode == 7:
            param(2, 1 if param(0) < param(1) else 0)
            ip += 4
        elif opcode == 8:
            param(2, 1 if param(0) == param(1) else 0)
            ip += 4
        elif opcode == 9:
            base += param(0)
            ip += 2
        elif opcode == 99:
            ip += 1
            break
        else:
            raise NotImplementedError("Illegal opcode")

    return memory

if __name__ == '__main__':
    print((run_pgm(load_pgm(sys.argv[1]))))

