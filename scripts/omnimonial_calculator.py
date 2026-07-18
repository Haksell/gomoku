from fractions import Fraction
from math import comb
import sys

assert len(sys.argv) == 2
n = int(sys.argv[1])
assert n > 0
assert n % 4 == 0

lim = Fraction(1, n)

p = Fraction(0)
k = n
while True:
    p += Fraction(comb(n, k), 1 << n)
    if p >= lim:
        break
    k -= 1

win_diff = k - n // 2

print(f"const GAMES_BY_EPOCH: usize = {n};")
print(f"const REQUIRED_WIN_DIFFERENTIAL: i32 = {win_diff};")
