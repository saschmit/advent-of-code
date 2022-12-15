#!/usr/bin/python

import sys

debug = False

inst = []
for line in open(sys.argv[1]):
    dir, dist = line.strip().split()
    inst.append((dir, int(dist)))

# Determine grid dimensions
lt = rt = up = dn = 0
r = c = 0
for dir, dist in inst:
    if dir == 'L':
        c -= dist
        lt = min(lt, c)
    elif dir == 'R':
        c += dist
        rt = max(rt, c)
    elif dir == 'U':
        r -= dist
        up = min(up, r)
    elif dir == 'D':
        r += dist
        dn = max(dn, r)
    else:
        assert False

#print("l = {}, r = {}, u = {}, d = {}".format(lt, rt, up, dn))
width = rt - lt + 1
height = dn - up + 1
#print("{} x {}".format(height, width))

# determine starting position
start = [-up, -lt]
#print(start)

# Generate grid
grid = [['.' for col in range(width)] for row in range(height)]

# Mark start
grid[start[0]][start[1]] = '#'

head = list(start)
tail = list(head)

def disp_grid(grid, start, head, tail):
    for row in range(height):
        out = ""
        for col in range(width):
            if head is not None and row == head[0] and col == head[1]:
                out += 'H'
            elif tail is not None and row == tail[0] and col == tail[1]:
                out += 'T'
            elif row == start[0] and col == start[1]:
                out += 's'
            else:
                out += grid[row][col]
        print(out)

def xlate(head, tail, dir):
    def is_connected(hd, tl):
        return abs(hd[0] - tl[0]) <= 1 and abs(hd[1] - tl[1]) <= 1
    assert is_connected(head, tail)

    new_head = list(head)
    new_tail = list(tail)

    if dir == 'U':
        new_head[0] = head[0] - 1
        if not is_connected(new_head, tail):
            new_tail[0] = tail[0] - 1
            new_tail[1] = new_head[1]
    elif dir == 'D':
        new_head[0] = head[0] + 1
        if not is_connected(new_head, tail):
            new_tail[0] = tail[0] + 1
            new_tail[1] = new_head[1]
    elif dir == 'L':
        new_head[1] = head[1] - 1
        if not is_connected(new_head, tail):
            new_tail[0] = new_head[0]
            new_tail[1] = tail[1] - 1
    elif dir == 'R':
        new_head[1] = head[1] + 1
        if not is_connected(new_head, tail):
            new_tail[0] = new_head[0]
            new_tail[1] = tail[1] + 1
    else:
        assert False

    assert is_connected(new_head, new_tail)

    return new_head, new_tail

if debug:
    dbg_print = print
else:
    dbg_print = lambda x=None: None
    disp_grid = lambda a, b, c, d: None

dbg_print("== Initial State ==")
dbg_print()
disp_grid(grid, start, head, tail)
for dir, dist in inst:
    dbg_print()
    dbg_print("== {} {} ==".format(dir, dist))
    for i in range(dist):
        dbg_print()
        head, tail = xlate(head, tail, dir)
        grid[tail[0]][tail[1]] = '#'
        disp_grid(grid, start, head, tail)

dbg_print()
dbg_print()
disp_grid(grid, start, None, None)

part1_count = 0
for row in range(height):
    for col in range(width):
        part1_count += 1 if grid[row][col] == '#' else 0

print("Part 1: {}".format(part1_count))
