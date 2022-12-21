#!/usr/bin/python

import sys
from operator import itemgetter

def set_debug(debug):
    global dbg_print
    if debug:
        dbg_print = print
    else:
        dbg_print = lambda x: None

class SparseMap:
    def __init__(self, fill):
        self._grid = {}
        self.fill = fill
        self._min_x = None
        self._max_x = None
        self._min_y = None
        self._max_y = None
    def draw(self, coord, value):
        self._grid[coord] = value
        x, y = coord
        self._min_x = min(self._min_x, x) if self._min_x is not None else x
        self._max_x = max(self._max_x, x) if self._max_x is not None else x
        self._min_y = min(self._min_y, y) if self._min_y is not None else y
        self._max_y = max(self._max_y, y) if self._max_y is not None else y
    def get(self, coord):
        return self._grid.get(coord, self.fill)
    def get_height(self):
        return self._max_y - self._min_y + 1
    def get_width(self):
        return self._max_x - self._min_x + 1
    def __str__(self):
        if self.get_height() * self.get_width() > 30*30:
            return "{:,} cells = {} x {}".format(self.get_width() * self.get_height(), self.get_width(), self.get_height())
        out = ""
        wdigits = max(len(str(self._min_x)), len(str(self._max_x)))
        hdigits = max(len(str(self._min_y)), len(str(self._max_y)))
        def digit(n, d):
            return "{:0{}}".format(n, wdigits)[d]
        for r in range(wdigits):
            out += ' ' * hdigits + ' '
            for c in range(self._min_x, self._max_x+1):
                out += digit(c, r)
            out += '\n'
        for r in range(self._min_y, self._max_y+1):
            out += "{:{}} ".format(r, hdigits)
            for c in range(self._min_x, self._max_x+1):
                out += self.get((c,r))
            out += '\n'
        return out.rstrip()

def calc_mdist(p1, p2):
    return abs(p1[0] - p2[0]) + abs(p1[1] - p2[1])

class SensorGrid:
    def __init__(self, input_data):
        self.map = SparseMap('.')
        self.sensors = {}
        self.beacons = []

        def extract_xy(data):
            x_plus, y_plus = data.split(', ')
            _, x = x_plus.split('=')
            _, y = y_plus.split('=')
            return int(x), int(y)

        for line in input_data.split('\n'):
            sensor_data, beacon_data = line.split(':')
            sx, sy = extract_xy(sensor_data)
            bx, by = extract_xy(beacon_data)
            self.map.draw((sx,sy), 'S')
            self.sensors[(sx,sy)] = calc_mdist((sx,sy),(bx,by))
            self.map.draw((bx,by), 'B')
            self.beacons.append((bx,by))
    def get_sensors(self):
        return self.sensors.keys()
    def find_row_coverage(self, row, bounds=None):
        x_ranges = []
        for coord, mdist in self.sensors.items():
            y_dist = abs(coord[1] - row)
            if y_dist <= mdist:
                x_dist = mdist - y_dist
                x_min = coord[0] - x_dist
                x_max = coord[0] + x_dist
                x_range = (x_min, x_max)
                if bounds is None:
                    x_ranges.append(x_range)
                elif range_overlap(x_range, bounds):
                    x_min = max(bounds[0], x_min)
                    x_max = min(bounds[1], x_max)
                    x_ranges.append((x_min, x_max))

        if not x_ranges:
            return None

        x_ranges.sort(key=itemgetter(0, 1))

        i = 0
        l = len(x_ranges)
        merged = False
        while i+1 < l:
            if range_overlap(x_ranges[i], x_ranges[i+1]) or range_adjacent(x_ranges[i], x_ranges[i+1]):
                x_ranges.insert(i, range_union(x_ranges.pop(i), x_ranges.pop(i)))
                l -= 1
            else:
                i += 1

        return x_ranges

    def count_part1(self, row, x_ranges):
        row_beacon_xs = set()
        for beacon in self.beacons:
            if beacon[1] == row:
                row_beacon_xs.add(beacon[0])

        gaps = 0
        for x_range in x_ranges:
            gaps += x_range[1] - x_range[0] + 1
            for row_beacon_x in row_beacon_xs:
                if x_range[0] <= row_beacon_x <= x_range[1]:
                    gaps -= 1

        return gaps

def range_union(r1, r2):
    assert range_overlap(r1, r2) or range_adjacent(r1, r2)
    return min(r1[0], r2[0]), max(r1[1], r2[1])

def range_overlap(r1, r2):
    return ((r2[0] <= r1[1] and r2[1] >= r1[1]) or
            (r1[0] <= r2[1] and r1[1] >= r2[1]))

def range_adjacent(r1, r2):
    return r1[1] + 1 == r2[0] or r2[1] + 1 == r1[0]

def range_invert(ranges, bounds):
    inv = []
    if bounds[0] < ranges[0][0]:
        inv.append((bounds[0], ranges[0][0]-1))

    for r in range(1, len(ranges)):
        inv.append((ranges[r-1][1]+1, ranges[r][0]-1))

    if ranges[-1][1] < bounds[0]:
        inv.append((ranges[-1][1]+1, bounds[1]))

    return inv

set_debug(False)
sg = SensorGrid(open(sys.argv[1]).read().rstrip())
dbg_print(sg.map)

if sys.argv[1] == 'sample':
    row = 10
else:
    row = 2000000

gaps = sg.count_part1(row, sg.find_row_coverage(row))

print("Part 1: {}".format(gaps))

if sys.argv[1] == 'sample':
    upper = 20
else:
    upper = 4000000

part2_pos = None
for row in range(upper):
    coverage = sg.find_row_coverage(row, (0, upper))
    if len(coverage) == 0:
        continue

    if len(coverage) == 1 and coverage[0][0] == 0 and coverage[0][1] == upper:
        continue

    gaps = range_invert(sg.find_row_coverage(row, (0, upper)), (0, upper))
    for gap in gaps:
        if gap[0] == gap[1]:
            part2_pos = (gap[0], row)

tune_freq = part2_pos[0] * 4000000 + part2_pos[1]
print("Part 2: {} -> {}".format(part2_pos, tune_freq))
