#!/usr/bin/python

from pprint import pprint

# Rules
# xM & yG, if x != y, then x will fry unless xG is also on the same level
# E can only move with 1 or 2 items
# E can only move 1 floor at a time
import sys
case = int(sys.argv[1])
if case == 0:
    init_state = set(['1:E', '1:HM', '1:LM', '2:HG', '3:LG' ])
    goal = set(['4:E', '4:HM', '4:LM', '4:HG', '4:LG' ])
elif case == 1:
    init_state = set(['1:E', '1:SG', '1:SM', '1:PG', '1:PM', '2:TG', '2:RG', '2:RM', '2:CG', '2:CM', '3:TM'])
    goal = set(['4:E', '4:SG', '4:SM', '4:PG', '4:PM', '4:TG', '4:RG', '4:RM', '4:CG', '4:CM', '4:TM'])
elif case == 2:
    init_state = set(['1:E', '1:SG', '1:SM', '1:PG', '1:PM', '2:TG', '2:RG', '2:RM', '2:CG', '2:CM', '3:TM','1:EG','1:EM','1:DG','1:DM'])
    goal = set(['4:E', '4:SG', '4:SM', '4:PG', '4:PM', '4:TG', '4:RG', '4:RM', '4:CG', '4:CM', '4:TM','4:EG','4:EM','4:DG','4:DM'])
else:
    raise ValueError

all_states = {}
active_floors = ['1', '2', '3', '4']

pick_two = {}
for k in xrange(1, len(goal)):
    pick_two[k] = set()
    for i in xrange(0, k):
        for j in xrange(0, k):
            pick_two[k].add(tuple(sorted((i, j))))

def key(state):
    state = sorted(state)
    for i in xrange(1, len(state)):
        if state[i][-1] == 'E':
            continue
        if state[i-1][:-1] == state[i][:-1]:
            state[i-1] = state[i-1][:2] + 'x' + state[i-1][-1]
            state[i] = state[i][:2] + 'x' + state[i][-1]
    count = 0
    m = {}
    for i in xrange(len(state)):
        if state[i][-1] == 'E':
            continue
        if state[i][2] not in m:
            m[state[i][2]] = count
            count += 1
    for i in xrange(len(state)):
        if state[i][-1] == 'E':
            continue
        state[i] = state[i][:2] + chr(ord('a') + m[state[i][2]]) + state[i][-1]
    return ",".join(sorted(state))

def legal_state(state):
    for f in map(str, xrange(1, 5)):
        floor = map(lambda x: x[2:], filter(lambda x: x[0] == f, state))
        if not floor:
            continue
        if 'E' in floor and len(floor) == 1:
            return False
        gens = set(map(lambda x: x[0], filter(lambda x: x[-1] == 'G', floor)))
        mcs = set(map(lambda x: x[0], filter(lambda x: x[-1] == 'M', floor)))
        if len(gens - mcs) and len(mcs - gens):
            return False
    return True

def print_state(state):
    for fl in ('4', '3', '2', '1'):
        floor = sorted(map(lambda s: s[2:], filter(lambda s: s[0] == fl, state)))
        if 'E' in floor:
            elevator = 'E'
            floor = filter(lambda s: s[-1] != 'E', floor)
        else:
            elevator = ' '
        print fl, elevator, floor

class Dupe(RuntimeError):
    pass

class Illegal(RuntimeError):
    pass

class StopSearch(RuntimeError):
    def __init__(self, gen, closed_floors):
        self.gen = gen
        self.closed_floors = closed_floors

class Node:
    def __init__(self, parent, state, gen):
        if not legal_state(state):
            raise Illegal()
        self.state = state
        self.parent = parent
        self.gen = gen
        self.done = False
        k = key(state)
        if k in all_states:
            if all_states[k].gen > gen:
                other = all_states[k]
                other.parent = parent
                other.gen = gen
            raise Dupe()
        else:
            all_states[k] = self

        if k == key(goal):
            raise StopSearch(gen, None)
        else:
            closed = []
            for fl in map(lambda f: str(active_floors[f]), xrange(len(active_floors))):
                if not filter(lambda x: fl in x, state):
                    closed.append(fl)
                else:
                    break
            if closed:
                raise StopSearch(gen, closed)

    def search(self):
        if self.done:
            return

        floors_to_go = set(map(str, xrange(min(map(lambda x: int(x[0]), self.state)), 5)))

        elevator = filter(lambda x: x[-1] == 'E', self.state)[0]
        floor_num = elevator[0]
        floor = filter(lambda x: x[0] == floor_num and x[-1] != 'E', self.state)
        floor.sort()
        floor_set = set(floor)
        #print self.state
        for offset in (1, -1):
            next_floor_num = str(int(floor_num) + offset)
            if next_floor_num not in floors_to_go:
                continue

            #print "moving from %s to %s" % (floor_num, next_floor_num)

            next_floor = filter(lambda x: x[0] == next_floor_num, self.state)
            next_floor_set = set(next_floor)
            other_floors = set(filter(lambda x: x[-1] != 'E', self.state)) - floor_set - next_floor_set
            #print sorted(floor)
            #print sorted(next_floor)
            #print sorted(other_floors)

            for choices in pick_two[len(floor)]:
                c = floor[choices[0]], floor[choices[1]]
                new_c = map(lambda x: next_floor_num + x[1:], c)
                new_floor = set(floor) - set(c)
                new_next_floor = set(next_floor) | set(new_c)
                new_state = new_floor | new_next_floor | other_floors | set([next_floor_num + ":E"])
                assert sorted(map(lambda s: s[2:], self.state)) == sorted(map(lambda s: s[2:], new_state))
                try:
                    Node(self, new_state, self.gen+1)
                except Illegal:
                    pass
                except Dupe:
                    pass
        self.done = True

assert legal_state(init_state)
assert legal_state(goal)

goal_key = key(goal)

Node(None, init_state, 0)
last_n = 0
while goal_key not in all_states and filter(lambda s: not all_states[s].done, all_states):
    search = filter(lambda s: not all_states[s].done, all_states)
    if len(all_states) - last_n > 1000:
        last_n = len(all_states)
        print last_n, len(search)
    try:
        map(lambda s: all_states[s].search(), search)
    except StopSearch as ss:
        if ss.closed_floors is not None:
            for closed in ss.closed_floors:
                print "Floor %s now closed. Pruning states" % closed
                del active_floors[0]
                prune = filter(lambda s: closed in s, all_states)
                for s in prune:
                    del all_states[s]

assert goal_key in all_states
print all_states[goal_key].gen
def d(n):
    if n.parent is not None:
        d(n.parent)
    print_state(n.state)
    print

d(all_states[goal_key])
