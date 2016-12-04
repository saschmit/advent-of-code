#!/usr/bin/python

import sys

def parseEncryptedRoom(s):
    assert s[-1] == ']'
    dashed, checksum = s[:-1].split('[')
    dashed = dashed.split('-')
    letters = dashed[:-1]
    sector = int(dashed[-1])

    return letters, sector, checksum

def computeChecksum(letters):
    freq = {}
    for letter in "".join(letters):
        if letter not in freq:
            freq[letter] = 1
        else:
            freq[letter] += 1

    ranks = {}
    for letter in freq:
        rank = freq[letter]
        if rank not in ranks:
            ranks[rank] = [ letter ]
        else:
            ranks[rank].append(letter)

    checksum = ""
    for rank in reversed(sorted(ranks.keys())):
        checksum += "".join(sorted(ranks[rank]))

    checksum = checksum[:5]
    return checksum

def isValid(roomTuple):
    letters, sector, checksum = roomTuple
    return computeChecksum(letters) == checksum

if __name__ == "__main__":
    if len(sys.argv) == 1:
        data = """aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]"""
    else:
        data = open(sys.argv[1], 'r').read()

    data = data.strip().split("\n")
    data = map(parseEncryptedRoom, data)
    data = filter(isValid, data)
    print(sum(map(lambda t: t[1], data)))
