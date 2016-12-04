#!/usr/bin/python

import sys

class KeyPadInstructions:
    __keypad = (
        ( '1', '2', '3' ),
        ( '4', '5', '6' ),
        ( '7', '8', '9' ),
    )
    def __init__(self, instr):
        self.__code = ""
        self.__pos = [1, 1]
        self.decode(instr)
    def __move(self, mv):
        xform = {
            'U': (-1,0),
            'D': (1,0),
            'L': (0,-1),
            'R': (0,1),
        }
        for i in xrange(0, len(self.__pos)):
            self.__pos[i] = max(0, min(2, self.__pos[i] + xform[mv][i]))
    def getCode(self):
        return self.__code
    def readKeypad(self):
        return self.__keypad[self.__pos[0]][self.__pos[1]]
    def decode(self, instr):
        lines = instr.strip().split("\n")
        for line in lines:
            for mv in line:
                self.__move(mv)
            self.__code += self.readKeypad()

if __name__ == "__main__":
    if len(sys.argv) == 1:
        print(KeyPadInstructions("""
ULL
RRDDD
LURDL
UUUUD
""").getCode())
    elif len(sys.argv) == 2:
        print(KeyPadInstructions(open(sys.argv[1],'r').read()).getCode())
    else:
        exit(1)
