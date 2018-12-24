#!/usr/bin/python3

import sys

lines = sys.stdin.readlines()
lines.sort()

events = []
for line in lines:
    if line.endswith("begins shift\n"):
        events.append('START')
    elif line.endswith("falls asleep\n"):
        events.append('SLEEP')
    elif line.endswith("wakes up\n"):
        events.append('WAKE')
    else:
        assert False

state = None
for event in events:
    if state is None:
        assert event == 'START'
        state = 'AWAKE'
    elif state == 'AWAKE':
        assert event != 'WAKE'
        if event == 'SLEEP':
            state = 'ASLEEP'
        elif event == 'START':
            state = 'AWAKE'
    elif state == 'ASLEEP':
        assert event == 'WAKE'
        state = 'AWAKE'
    else:
        assert False

