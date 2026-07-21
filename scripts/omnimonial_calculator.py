from math import comb
import sys

assert len(sys.argv) == 2
n = int(sys.argv[1])
assert n > 0
assert n % 4 == 0

lim = (1 << n) // n

p = 0
k = n
while True:
    p += comb(n, k)
    if p >= lim:
        break
    k -= 1

win_diff = k - n // 2

print(f"const GAMES_BY_EPOCH: usize = {n};")
print(f"const REQUIRED_WIN_DIFFERENTIAL: i32 = {win_diff};")
