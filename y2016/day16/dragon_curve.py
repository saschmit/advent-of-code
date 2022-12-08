#!/usr/bin/python

def invert_flip(bits):
    out = ""
    for i in xrange(len(bits)-1, -1, -1):
        bit = bits[i]
        if bit == '0':
            out += '1'
        else:
            assert bit == '1'
            out += '0'
    return out


def dragon_curve(bits):
    return bits + '0' + invert_flip(bits)

assert dragon_curve("1") == "100"
assert dragon_curve("0") == "001"
assert dragon_curve("11111") == "11111000000"
assert dragon_curve("111100001010") == "1111000010100101011110000"

def fill_to_size(data, size):
    while len(data) < size:
        data = dragon_curve(data)
    return data[:size]

def compute_checksum(data):
    while len(data) % 2 == 0:
        checksum = ""
        for i in xrange(0, len(data), 2):
            if data[i] == data[i+1]:
                checksum += '1'
            else:
                checksum += '0'
        data = checksum
    return data

assert compute_checksum('110010110100') == '100'

assert compute_checksum(fill_to_size('10000', 20)) == '01100'

print compute_checksum(fill_to_size('10011111011011001', 272))
print compute_checksum(fill_to_size('10011111011011001', 35651584))
