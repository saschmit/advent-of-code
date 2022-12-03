#!/usr/bin/python

import sys

PTS_LOSE = 0
PTS_DRAW = 3
PTS_WIN = 6

PLAY_ROCK = 0
PLAY_PAPER = 1
PLAY_SCISSORS = 2

def score_round(opp, you):
    if opp == you:
        return PTS_DRAW
    if you == (opp + 1) % 3:
        return PTS_WIN
    if opp == (you + 1) % 3:
        return PTS_LOSE

def score_play(opp, you):
    return you + 1

decoder = {
'A': PLAY_ROCK,
'B': PLAY_PAPER,
'C': PLAY_SCISSORS,
'X': PLAY_ROCK,
'Y': PLAY_PAPER,
'Z': PLAY_SCISSORS,
}

pretty = ['rock', 'paper', 'scissors']

total = 0
for line in open(sys.argv[1]).readlines():
    #print("line: {}".format(line.strip()))
    opp, you = [decoder[x] for x in line.strip().split()]
    #print("opp = {}, you = {}".format(pretty[opp], pretty[you]))
    round_score = score_round(opp, you)
    play_score = score_play(opp, you)
    #print("total = {} (round = {} + play = {})".format(round_score + play_score, round_score, play_score))
    total += round_score + play_score

print("total = {}".format(total))
