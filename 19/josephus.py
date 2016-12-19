#!/usr/bin/python

def g(n, k):
    if n == 1:
        return 0
    elif 1 < n < k:
        return (g(n-1, k) + k) % n
    else:
        np = n - (n//k)
        return k*(((g(np, k) - n) % k) % np) // (k-1)

print g(3001330, 1)
