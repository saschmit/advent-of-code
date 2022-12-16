#!/usr/bin/python

import sys

def disp_grid(grid, start, rope):
    for row in range(len(grid)):
        out = ""
        for col in range(len(grid[0])):
            char = grid[row][col]

            if row == start[0] and col == start[1]:
                char = 's'

            if rope is not None:
                for knot in range(len(rope)-1, -1, -1):
                    if row == rope[knot][0] and col == rope[knot][1]:
                        char = str(knot)
                if char == '0':
                    char = 'H'
                if len(rope) == 2 and char == '1':
                    char = 'T'

            out += char

        print(out)

def heal(head, tail):
    def is_connected(hd, tl):
        return abs(hd[0] - tl[0]) <= 1 and abs(hd[1] - tl[1]) <= 1

    xlate = [
    # col    +0        +1        +2        -2        -1
        [ (+0, +0), (+0, +0), (+0, +1), (+0, -1), (+0, +0) ], # row + 0
        [ (+0, +0), (+0, +0), (+1, +1), (+1, -1), (+0, +0) ], # row + 1
        [ (+1, +0), (+1, +1), (+1, +1), (+1, -1), (+1, -1) ], # row + 2
        [ (-1, +0), (-1, +1), (-1, +1), (-1, -1), (-1, -1) ], # row - 2
        [ (+0, +0), (+0, +0), (-1, +1), (-1, -1), (+0, +0) ]  # row - 1
    ]

    delta = xlate[head[0] - tail[0]][head[1] - tail[1]]

    new_tail = [None, None]
    new_tail[0] = tail[0] + delta[0]
    new_tail[1] = tail[1] + delta[1]

    assert is_connected(head, new_tail)

    return new_tail

def xlate_rope(rope, dir):
    new_rope = list(rope)
    if dir == 'U':
        new_rope[0][0] -= 1
    elif dir == 'D':
        new_rope[0][0] += 1
    elif dir == 'L':
        new_rope[0][1] -= 1
    elif dir == 'R':
        new_rope[0][1] += 1
    else:
        assert False

    for knot in range(1, len(rope)):
        new_rope[knot] = heal(new_rope[knot-1], new_rope[knot])

    return new_rope

debug = False
if debug:
    dbg_print = print
else:
    dbg_print = lambda x=None: None
    disp_grid = lambda a, b, c: None

# Load instructions
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

dbg_print("l = {}, r = {}, u = {}, d = {}".format(lt, rt, up, dn))
width = rt - lt + 1
height = dn - up + 1
dbg_print("{} x {}".format(height, width))

# determine starting position
start = (-up, -lt)
dbg_print(start)

def solve(height, width, start, rope_len):
    # Generate grid
    grid = [['.' for col in range(width)] for row in range(height)]

    # Mark start
    grid[start[0]][start[1]] = '#'

    rope = []
    for knot in range(rope_len):
        rope.append(list(start))

    dbg_print("== Initial State ==")
    dbg_print()
    disp_grid(grid, start, rope)
    for dir, dist in inst:
        dbg_print()
        dbg_print("== {} {} ==".format(dir, dist))
        for i in range(dist):
            dbg_print()
            rope = xlate_rope(rope, dir)
            grid[rope[-1][0]][rope[-1][1]] = '#'
            disp_grid(grid, start, rope)

    dbg_print()
    dbg_print()
    disp_grid(grid, start, None)

    count = 0
    for row in range(height):
        for col in range(width):
            count += 1 if grid[row][col] == '#' else 0
    return count

part1_count = solve(height, width, start, 2)
print("Part 1: {}".format(part1_count))

part2_count = solve(height, width, start, 10)
print("Part 2: {}".format(part2_count))
