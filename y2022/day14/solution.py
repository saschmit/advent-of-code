#!/usr/bin/python

import sys

def set_debug(debug):
    global dbg_print
    if debug:
        dbg_print = print
    else:
        dbg_print = lambda x: None

class GrowableUndergroundMap:
    def __init__(self, origin, fill):
        self._grid = [['']]
        self._height = 1
        self._width = 1
        self._offset_x = origin[0]
        self._offset_y = origin[1]
        self.fill = fill
        self.draw(origin, fill)
    def __grow(self, dir, amt):
        dbg_print("grow({}, {})".format(dir, amt))
        if dir == 'left':
            for row in range(self._height):
                self._grid[row] = [self.fill] * amt + self._grid[row]
            self._offset_x -= amt
            self._width += amt
        elif dir == 'right':
            for row in range(self._height):
                self._grid[row] = self._grid[row] + [self.fill] * amt
            self._width += amt
        elif dir == 'down':
            for _ in range(amt):
                self._grid = self._grid + [[self.fill] * self._width]
            self._height += amt
        elif dir == 'up':
            for _ in range(amt):
                self._grid = [[self.fill] * self._width] + self._grid
            self._offset_y -= amt
            self._height += amt
        else:
            raise ValueError("invalid direction")
    def __xlate_coords_in(self, c):
        return c[1] - self._offset_y, c[0] - self._offset_x
    def draw(self, coord, value):
        dbg_print("draw({}, {})".format(coord, value))
        int_coord = self.__xlate_coords_in(coord)
        if int_coord[0] < 0:
            self.__grow('up', -int_coord[0])
        if int_coord[0] >= self._height:
            self.__grow('down', int_coord[0] - self._height + 1)
        if int_coord[1] < 0:
            self.__grow('left', -int_coord[1])
            int_coord = self.__xlate_coords_in(coord)
        if int_coord[1] >= self._width:
            self.__grow('right', int_coord[1] - self._width + 1)
        assert 0 <= int_coord[0] < self._height
        assert 0 <= int_coord[1] < self._width

        self._grid[int_coord[0]][int_coord[1]] = value
    def get(self, coord):
        int_coord = self.__xlate_coords_in(coord)
        if not (0 <= int_coord[0] < self._height):
            return self.fill
        if not (0 <= int_coord[1] < self._width):
            return self.fill

        return self._grid[int_coord[0]][int_coord[1]]
    def get_height(self):
        return self._offset_y + self._height
    def get_width(self):
        return self._offset_x + self._width
    def __str__(self):
        #return "\n".join(["".join(self._grid[row]) for row in range(len(self._grid))])
        out = ""
        wdigits = len(str(self.get_width()-1))
        hdigits = len(str(self.get_height()-1))
        def digit(n, d):
            return str(n // 10**(wdigits-d-1) % 10)
        for r in range(wdigits + self._height):
            if r < wdigits:
                out += ' ' * hdigits + ' '
                for c in range(self._offset_x, self._width + self._offset_x):
                    out += digit(c, r)
            else:
                out += ("{:%d} " % hdigits).format(r-wdigits)
                for c in range(self._offset_x, self._width + self._offset_x):
                    out += self.get((c,r-wdigits))
            out += '\n'
        return out.rstrip()

class SandTracker:
    def __init__(self, input_data, floor=False):
        self.map = GrowableUndergroundMap((500, 0), '.')
        self.map.draw((500, 0), '+')
        self._floor_lvl = None

        rocks = []
        for line in input_data.split('\n'):
            prev = None
            for coord in line.split(' -> '):
                cur = [int(n) for n in coord.split(',')]
                if prev is not None:
                    rocks.append((prev, cur))
                prev = cur

        def draw_rock(canvass, start, end):
            start_x, start_y = start
            end_x, end_y = end
            assert start_x == end_x or start_y == end_y
            if end_x < start_x:
                start_x, end_x = end_x, start_x
            if end_y < start_y:
                start_y, end_y = end_y, start_y
            for x in range(start_x, end_x+1):
                for y in range(start_y, end_y+1):
                    canvass.draw((x,y), '#')

        for start, end in rocks:
            draw_rock(self.map, start, end)

        if floor:
            max_depth = 0
            leftmost = 1000000
            rightmost = 0
            for start, end in rocks:
                max_depth = max(max_depth, start[1], end[1])
                leftmost = min(leftmost, start[0], end[0])
                rightmost = max(rightmost, start[0], end[0])

            self._floor_lvl = max_depth + 2
            for x in range(leftmost, rightmost+1):
                self.map.draw((x, self._floor_lvl), '=')

    def add_sand(self):
        at_rest = False
        in_abyss = False
        source_blocked = False
        last = None
        x, y = 500, 0
        deltas = ( (+0, +1), (-1, +1), (+1, +1) )
        trace = []
        while not at_rest and not in_abyss and not source_blocked:
            if self.map.get((500,0)) == 'o':
                source_blocked = True
                continue

            if last == (x, y):
                dbg_print("grain at rest at {}, replacing {}".format((x,y), self.map.get((x,y))))
                self.map.draw((x,y), 'o')
                at_rest = True
                continue

            last = x, y
            trace.append(last)

            for dx, dy in deltas:
                dbg_print("applying delta {} to {}".format((dx,dy), (x,y)))
                cand = (x+dx, y+dy)
                if self._floor_lvl is not None:
                    if self.map.get((x+dx, self._floor_lvl)) == self.map.fill:
                        self.map.draw((x+dx, self._floor_lvl), '=')
                if cand[1] >= self.map.get_height():
                    dbg_print("{} is below map floor of {} -- to the abyss!".format(cand, self.map.get_height()-1))
                    in_abyss = True
                    break
                if self.map.get(cand) == self.map.fill:
                    dbg_print("{} ({}) is ok".format(cand, self.map.get(cand)))
                    x, y = cand
                    break
                else:
                    dbg_print("obstruction at {} ({})".format(cand, self.map.get(cand)))

        if in_abyss:
            for pos in trace:
                assert self.map.get(pos) in (self.map.fill, '+')
                self.map.draw(pos, '~')

        return not (in_abyss or source_blocked)

set_debug(False)
st = SandTracker(open(sys.argv[1]).read().rstrip())
dbg_print(st.map)
more_sand = True
grains = 0
while more_sand:
    more_sand = st.add_sand()
    if more_sand:
        grains += 1
        dbg_print("Grain {}".format(grains))
        dbg_print(st.map)
    else:
        dbg_print("Final:")
        dbg_print(st.map)

print("Part 1: {}".format(grains))

st = SandTracker(open(sys.argv[1]).read().rstrip(), True)
dbg_print(st.map)
more_sand = True
grains = 0
while more_sand:
    more_sand = st.add_sand()
    if more_sand:
        grains += 1
        dbg_print("Grain {}".format(grains))
        dbg_print(st.map)
    else:
        dbg_print("Final:")
        dbg_print(st.map)

print("Part 2: {}".format(grains))
