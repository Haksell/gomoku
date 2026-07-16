def swap_12(n):
    return 0 if n == 0 else 3 - n


def trits(n):
    return (
        n % 3,
        n // 3 % 3,
        n // 9 % 3,
        n // 27 % 3,
        n // 81 % 3,
        n // 243 % 3,
    )


def sym(n):
    x0, x1, x2, x3, x4, x5 = trits(n)
    return x5 + 3 * x4 + 9 * x3 + 27 * x2 + 81 * x1 + 243 * x0


def opp(n):
    x0, x1, x2, x3, x4, x5 = trits(n)
    return (
        swap_12(x0)
        + 3 * swap_12(x1)
        + 9 * swap_12(x2)
        + 27 * swap_12(x3)
        + 81 * swap_12(x4)
        + 243 * swap_12(x5)
    )


def sym_opp(n):
    return sym(opp(n))


indices = set()

for n in range(729):
    if n != sym_opp(n):
        indices.add(min(n, sym(n), opp(n), sym_opp(n)))

indices = sorted(indices)
print(len(indices))
print(indices)
print(list(map(sym, indices)))
print(list(map(opp, indices)))
print(list(map(sym_opp, indices)))
