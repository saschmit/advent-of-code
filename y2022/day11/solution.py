#!/usr/bin/python

import sys
from pprint import pprint

debug = False
if debug:
    dbg_print = print
    dbg_pprint = pprint
else:
    dbg_print = lambda x: None
    dbg_pprint = lambda x: None

class Item:
    def __init__(self, worry_level):
        self._worry = int(worry_level)
    def inspect(self, op, arg, reducor):
        if arg == 'old':
            arg = self._worry

        if op == '+':
            self._worry += arg
            dbg_print("    Worry level increases by {} to {}.".format(arg, self._worry))
        elif op == '*':
            self._worry *= arg
            dbg_print("    Worry level is multiplied by {} to {}.".format(arg, self._worry))
        else:
            assert False

        self._worry = reducor(self._worry)
    def test(self, divisor):
        return self._worry % divisor == 0

    def __str__(self):
        return "{}".format(self._worry)

    def __repr__(self):
        return "Item({})".format(self)

class Monkey:
    def __init__(self, monkeydata):
        lines = [line.strip() for line in monkeydata.split('\n')]

        # Monkey number
        self._num = int(lines[0][:-1].split()[1])

        # Starting items
        _, items = lines[1].split(': ')
        self._items = [Item(item) for item in items.split(', ')]

        # Operation
        _, _, _, _, self._op, self._arg2 = lines[2].split()
        try:
            self._arg2 = int(self._arg2)
        except ValueError:
            pass

        # Test
        self._div = int(lines[3].split()[3])

        # Throws
        t = lines[4].split()[5]
        f = lines[5].split()[5]
        self._throw = {
            True: int(t),
            False: int(f),
        }

        self._inspect_cnt = 0

    def take_turn(self, monkeys, reducor):
        while True:
            try:
                item = self._items.pop(0)
                
                dbg_print("  Monkey inspects an item with a worry level of {}.".format(item._worry))
                item.inspect(self._op, self._arg2, reducor)
                self._inspect_cnt += 1

                dbg_print("    Current worry level {} divisible by {}.".format("is" if item.test(self._div) else "is not", self._div))
                monkeys[self._throw[item.test(self._div)]].catch(item)
                dbg_print("    Item with worry level {} is thrown to monkey {}.".format(item._worry, self._throw[item.test(self._div)]))
            except IndexError:
                break

    def catch(self, item):
        self._items.append(item)

    def get_count(self):
        return self._inspect_cnt

    def __str__(self):
        return "{}: {}; {}={}; /{}; -> {}, {}".format(self._num, self._items, self._op, self._arg2, self._div, self._throw[True], self._throw[False])
    def __repr__(self):
        return "Monkey({})".format(self)

def load_input(filename):
    data = open(filename).read()
    monkeydata = data.strip().split('\n\n')

    monkeys = []
    for monkeydatum in monkeydata:
        monkey = Monkey(monkeydatum)

        assert monkey._num == len(monkeys)

        monkeys.append(monkey)
        
    return monkeys

def play_round(monkeys, reducor):
    for monkey in monkeys:
        dbg_print("Monkey {}".format(monkey._num))
        monkey.take_turn(monkeys, reducor)

monkeys = load_input(sys.argv[1])
dbg_pprint(monkeys)

def div3(n):
    dbg_print("    Monkey gets bored with item. Worry level is divided by {} to {}.".format(3, n // 3))
    return n // 3

for round in range(1, 21):
    play_round(monkeys, div3)
    dbg_print("After round {}, the monkeys are holding items with these worry levels:".format(round))
    dbg_pprint(monkeys)

for monkey in monkeys:
    dbg_print("Monkey {} inspected items {} times.".format(monkey._num, monkey.get_count()))

inspection_rates = [monkey.get_count() for monkey in monkeys]
inspection_rates.sort()
monkey_business = inspection_rates[-2] * inspection_rates[-1]

print("Part 1: {}".format(monkey_business))

monkeys = load_input(sys.argv[1])

reduction_factor = 1
for monkey in monkeys:
    reduction_factor *= monkey._div

for round in range(1, 10001):
    play_round(monkeys, lambda n: n % reduction_factor)
    dbg_print("After round {}, the monkeys are holding items with these worry levels:".format(round))
    if round in [1, 20, 1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 10000]:
        dbg_print([monkey.get_count() for monkey in monkeys])

inspection_rates = [monkey.get_count() for monkey in monkeys]
inspection_rates.sort()
monkey_business = inspection_rates[-2] * inspection_rates[-1]

print("Part 2: {}".format(monkey_business))
