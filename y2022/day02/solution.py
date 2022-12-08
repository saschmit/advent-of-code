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

def get_play(opp, result):
    if result == PTS_DRAW:
        return opp
    if result == PTS_LOSE:
        return (opp + 3 - 1) % 3
    if result == PTS_WIN:
        return (opp + 1) % 3

decoder1 = {
'A': PLAY_ROCK,
'B': PLAY_PAPER,
'C': PLAY_SCISSORS,
'X': PLAY_ROCK,
'Y': PLAY_PAPER,
'Z': PLAY_SCISSORS,
}

decoder2 = {
'A': PLAY_ROCK,
'B': PLAY_PAPER,
'C': PLAY_SCISSORS,
'X': PTS_LOSE,
'Y': PTS_DRAW,
'Z': PTS_WIN,
}

pretty = ['rock', 'paper', 'scissors']
pretty_res = {
PTS_LOSE: 'lose',
PTS_WIN: 'win',
PTS_DRAW: 'draw'
}

total1 = 0
total2 = 0
for line in open(sys.argv[1]).readlines():
    #print("line: {}".format(line.strip()))
    opp, you = [decoder1[x] for x in line.strip().split()]
    #print("(pt1) opp = {}, you = {}".format(pretty[opp], pretty[you]))
    play_score = score_play(opp, you)
    round_score = score_round(opp, you)
    #print("(pt1) total = {} (play = {} + round = {})".format(play_score + round_score, play_score, round_score))
    total1 += play_score + round_score

    #print("line: {}".format(line.strip()))
    opp, result = [decoder2[x] for x in line.strip().split()]
    you = get_play(opp, result)
    #print("(pt2) opp = {}, result = {}, you = {}".format(pretty[opp], pretty_res[result], pretty[you]))
    play_score = score_play(opp, you)
    round_score = score_round(opp, you)
    #print("(pt2) total = {} (play = {} + round = {})".format(play_score + round_score, play_score, round_score))
    total2 += play_score + round_score

print("total (pt1) = {}".format(total1))
print("total (pt2) = {}".format(total2))
