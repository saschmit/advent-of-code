#!/usr/bin/python

def incr(passwd):
    decoded = map(lambda x: ord(x) - ord('a'), reversed(passwd))
    carry = 1
    for i in xrange(len(decoded)):
        carry, decoded[i] = divmod(decoded[i] + carry, 26)
        if not carry:
            break
    assert not carry
    if carry:
        decoded.append(0)
    return "".join(map(lambda n: chr(n + ord('a')), reversed(decoded)))

assert incr("xx") == "xy"
assert incr("xy") == "xz"
assert incr("xz") == "ya"
assert incr("ya") == "yb"

def is_straight(triple):
    a2x = "abcdefghijklmnopqrstuvwx"
    assert len(set(a2x)) == 24

    assert len(triple) == 3
    if triple[0] not in a2x:
        return False

    for i in xrange(2):
        if incr(triple[i]) != triple[i+1]:
            return False

    return True

def has_straight(passwd):
    for i in xrange(len(passwd)-2):
        if is_straight(passwd[i:i+3]):
            return True
    return False

def legal_chars(passwd):
    return set(passwd) & set("iol") == set()

def has_pairs(passwd):
    pairs = 0
    skip = 0
    for i in xrange(len(passwd)-1):
        if skip:
            skip -= 1
            continue
        if passwd[i] == passwd[i+1]:
            skip = 1
            pairs += 1

    return pairs > 1

def is_valid(passwd):
    return legal_chars(passwd) and has_straight(passwd) and has_pairs(passwd)

def incr_passwd(passwd):
    passwd = incr(passwd)
    while not is_valid(passwd):
        passwd = incr(passwd)
    return passwd

assert has_straight("hijklmmn") and not legal_chars("hijklmmn")
assert not has_straight("abbceffg") and has_pairs("abbceffg")
assert not has_pairs("abbcegjk")
assert incr_passwd("abcdefgh") == "abcdffaa"
assert incr_passwd("ghijklmn") == "ghjaabcc"

print "Self-test passed"
passwd = "hxbxwxba"
passwd = incr_passwd(passwd)
print passwd
passwd = incr_passwd(passwd)
print passwd
