#!/usr/bin/python

import sys

class Reindeer:
    def __init__(self, desc):
        (self.name, _, _, speed, speed_unit, _, fly_time, fly_time_unit,
            _, _, _, _, _, rest_time, rest_time_unit) = desc.strip().split(' ')

        assert speed_unit == 'km/s'
        assert fly_time_unit == 'seconds,'
        assert rest_time_unit == 'seconds.'

        self.__params = {
            'fly': { 'speed': int(speed), 'time': int(fly_time) },
            'rest': { 'speed': 0, 'time': int(rest_time) },
        }

    def __next_state(self):
        next_state = { 'fly': 'rest', 'rest': 'fly' }
        self.__state = next_state[self.__state]
        param = self.__params[self.__state]
        return param['speed'], param['time']

    def race(self, race_time):
        distance = 0
        self.__state = 'rest'

        while race_time:
            speed, state_duration = self.__next_state()

            timespan = min(race_time, state_duration)
            distance += timespan * speed
            race_time -= timespan

        return distance

flock = []
for line in sys.stdin.readlines():
    flock.append(Reindeer(line))

race_duration = int(sys.argv[1])
max_dist = 0
for reindeer in flock:
    dist = reindeer.race(race_duration)
    print "%s has gone %d km" % (reindeer.name, dist)
    if dist > max_dist:
        winner = reindeer.name
        max_dist = dist

print "%s wins with %d km" % (winner, max_dist)
