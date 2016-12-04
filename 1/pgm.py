#!/usr/bin/python

import sys

directions = []

class Map:
    def __init__(self, directions=""):
        self.__pos = [0, 0]
        self.__xform = (0, 1)
        self.apply(directions)
    def turnLeft(self):
        rot = {
            (0, 1): (-1, 0),
            (-1, 0): (0, -1),
            (0, -1): (1, 0),
            (1, 0): (0, 1)
        }
        self.__xform = rot[self.__xform]
    def turnRight(self):
        rot = {
            (0, 1): (1, 0),
            (1, 0): (0, -1),
            (0, -1): (-1, 0),
            (-1, 0): (0, 1)
        }
        self.__xform = rot[self.__xform]
    def go(self, n):
        for i in xrange(0, len(self.__pos)):
            self.__pos[i] += self.__xform[i] * n
    def getDistance(self):
        return sum(map(abs, self.__pos))
    def apply(self, directions):
        moves = directions.strip().split(", ")
        for move in moves:
            turn, fwd = move[0], int(move[1:])
            if turn == 'L':
                self.turnLeft()
                self.go(fwd)
            elif turn == 'R':
                self.turnRight()
                self.go(fwd)
            else:
                raise ValueError("unknown direction: " + turn)

if __name__ == "__main__":
    if len(sys.argv) == 1:
        print("Distance is: %d" % Map("R2, L3").getDistance())
        print("Distance is: %d" % Map("R2, R2, R2").getDistance())
        print("Distance is: %d" % Map("R5, L5, R5, R3").getDistance())
    elif len(sys.argv) == 2:
        print("Distance is: %d" % Map(sys.argv[1]).getDistance())
