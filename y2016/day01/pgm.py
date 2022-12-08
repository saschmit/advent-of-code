#!/usr/bin/python

import sys

directions = []

class Map:
    def __init__(self, directions="", stopAfter=None):
        self.__pos = [0, 0]
        self.__xform = (0, 1)
        self.__visited = {}
        self.__visit(self.__pos)
        self.apply(directions, stopAfter)
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
    def go(self, n, stopAfter=None):
        for count in xrange(0, n):
            for i in xrange(0, len(self.__pos)):
                self.__pos[i] += self.__xform[i]

            self.__visit(self.__pos)
            if stopAfter is not None:
                if self.__isVisited(self.__pos, stopAfter):
                    return False
        return True
    def getDistance(self):
        return sum(map(abs, self.__pos))
    def apply(self, directions, stopAfter=None):
        moves = directions.strip().split(", ")
        for move in moves:
            turn, fwd = move[0], int(move[1:])
            if turn == 'L':
                self.turnLeft()
            elif turn == 'R':
                self.turnRight()
            else:
                raise ValueError("unknown direction: " + turn)

            if not self.go(fwd, stopAfter):
                return
    def __visit(self, pos):
        if type(pos) is type([]):
            pos = tuple(pos)
        if pos not in self.__visited:
            self.__visited[pos] = 0
        self.__visited[pos] += 1
    def __isVisited(self, pos, times):
        if type(pos) is type([]):
            pos = tuple(pos)
        return self.__visited[pos] == times

if __name__ == "__main__":
    if len(sys.argv) == 1:
        print("Distance is: %d" % Map("R2, L3").getDistance())
        print("Distance is: %d" % Map("R2, R2, R2").getDistance())
        print("Distance is: %d" % Map("R5, L5, R5, R3").getDistance())
        print("Distance is: %d" % Map("R8, R4, R4, R8", 2).getDistance())
    elif len(sys.argv) == 2:
        print("Distance is: %d" % Map(sys.argv[1]).getDistance())
    elif len(sys.argv) == 3:
        print("Distance is: %d" % Map(sys.argv[1], int(sys.argv[2])).getDistance())
