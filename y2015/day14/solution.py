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

        self.__state = 'rest'
        self.distance = 0

        self.__next_state()

    def __next_state(self):
        next_state = { 'fly': 'rest', 'rest': 'fly' }
        self.__state = next_state[self.__state]
        param = self.__params[self.__state]
        self.__speed = param['speed']
        self.__time_left = param['time']

    def race(self):
        if not self.__time_left:
            self.__next_state()

        self.distance += 1 * self.__speed
        self.__time_left -= 1

flock = []
for line in sys.stdin.readlines():
    flock.append(Reindeer(line))

race_duration = int(sys.argv[1])
score = {}
for r in flock:
    score[r.name] = 0

winner = None
for i in xrange(1, race_duration+1):
    max_dist = 0
    for reindeer in flock:
        reindeer.race()
        if reindeer.distance > max_dist:
            winner = reindeer.name
            max_dist = reindeer.distance

    print "%d: %s takes a point" % (i, winner)
    score[winner] += 1

winner = None
max_score = 0
for r in score:
    print "%s has a score of %d points" % (r, score[r])
    if score[r] > max_score:
        max_score = score[r]
        winner = r
print "%s wins with %d points" % (winner, max_score)
