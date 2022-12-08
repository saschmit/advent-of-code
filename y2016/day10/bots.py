#!/usr/bin/python

import sys
from pprint import pprint

bots = {}
outputs = {}

class Bot:
    def __init__(self, num):
        self.__num = num
        self.__inputs = []
        self.__outputs = []
        self.__done = False
    def set_outputs(self, lo, hi):
        self.__outputs = [lo, hi]
    def give(self, var):
        self.__inputs.append(var)
    def execute(self):
        if len(self.__inputs) == 2:
            x, y = self.__inputs
            if (x, y) == (17, 61) or (y, x) == (17, 61):
                print "I am %s" % self.__num

            if x > y:
                if self.__outputs[1][0] == 'b':
                    bots[self.__outputs[1]].give(x)
                else:
                    outputs[self.__outputs[1]] = x
                if self.__outputs[0][0] == 'b':
                    bots[self.__outputs[0]].give(y)
                else:
                    outputs[self.__outputs[0]] = y
            else:
                if self.__outputs[0][0] == 'b':
                    bots[self.__outputs[0]].give(x)
                else:
                    outputs[self.__outputs[0]] = x
                if self.__outputs[1][0] == 'b':
                    bots[self.__outputs[1]].give(y)
                else:
                    outputs[self.__outputs[1]] = y
            self.__inputs = []
            self.__done = True
    def __str__(self):
        if not self.__done:
            return "%s has %s to give to %s" % (self.__num, self.__inputs, self.__outputs)
        else:
            return "%s is done" % self.__num
    def is_done(self):
        return self.__done


for line in sys.stdin.readlines():
    tokens = line.strip().split(' ')

    if tokens[0] == 'value':
        bot = None
        out = ' '.join(tokens[-2:])
        if out not in bots:
            bot = Bot(out)
            bots[out] = bot
        else:
            bot = bots[out]
        bot.give(int(tokens[1]))
    elif tokens[0] == 'bot':
        bot = None
        out = ' '.join(tokens[0:2])
        low = ' '.join(tokens[5:7])
        high = ' '.join(tokens[-2:])
        if out not in bots:
            bot = Bot(out)
            bots[out] = bot
        else:
            bot = bots[out]
        bot.set_outputs(low, high)
    else:
        raise ValueError("unknown line")

#pprint(map(str, bots.values()))
#pprint(outputs)

try:
    all_done = False
    while not all_done:
        all_done = True
        #pprint(map(str, bots.values()))
        #pprint(outputs)
        for bot in bots.values():
            bot.execute()
            all_done = all_done and bot.is_done()
except RuntimeError:
    pass

print outputs['output 0'] * outputs['output 1'] * outputs['output 2']
