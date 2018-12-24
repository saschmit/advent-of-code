#!/usr/bin/python3

import sys

lines = sys.stdin.readlines()
lines.sort()

logs = {}
current = None
today = None
start = None
end = None
for line in lines:
    if line.endswith("begins shift\n"):
        current = int(line.split('#')[1].split(' ')[0])
    elif line.endswith("falls asleep\n"):
        today = line[6:11]
        start = int(line[15:17])
    elif line.endswith("wakes up\n"):
        today = line[6:11]
        end = int(line[15:17])

        assert start < end

        if current not in logs:
            logs[current] = []
        logs[current].append((today, start, end))
    else:
        assert False

from pprint import pprint
#pprint(logs)

print("Date   ID     Minute")
print("              000000000011111111112222222222333333333344444444445555555555")
print("              012345678901234567890123456789012345678901234567890123456789")
sleepiness = {}
charts = {}
for guard in sorted(logs):
    asleep = 0
    for today, start, end in logs[guard]:
        chart = ""
        asleep += end - start
        for minute in range(0, 60):
            if minute < start:
                chart += '.'
            elif minute < end:
                chart += '#'
            else:
                chart += '.'
        print("{}  #{:4d}  {}".format(today, guard, chart))
        if guard not in charts:
            charts[guard] = [ chart ]
        else:
            charts[guard].append(chart)
    sleepiness[guard] = asleep
    print("({} minutes)".format(asleep))
    print()

most = -1
worst = None
for guard in sleepiness:
    if sleepiness[guard] > most:
        most = sleepiness[guard]
        worst = guard
    elif sleepiness[guard] == most:
        print("{} is as bad as {}".format(guard, worst))

pprint(sleepiness)

print("{} is the worst guard at {} minutes asleep".format(worst, most))
pprint(charts[worst])

summation = []
for minute in range(0, 60):
    total = 0
    for chart in charts[worst]:
        if chart[minute] == '#':
            total += 1
    summation.append(total)

worst_min = summation.index(max(summation))
print("{}'s worst minute is 00:{:02d}".format(worst, worst_min))

print("Part 1: {}".format(worst * worst_min))
