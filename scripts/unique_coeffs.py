STENCIL_SIZE = 7


def sym(n):
    return sum(n // 3**i % 3 * 3 ** (STENCIL_SIZE - i - 1) for i in range(STENCIL_SIZE))


def swap_12(n):
    return 0 if n == 0 else 3 - n


def opp(n):
    return sum(swap_12(n // 3**i % 3) * 3**i for i in range(STENCIL_SIZE))


def sym_opp(n):
    return sym(opp(n))


indices = sorted(
    {
        min(n, sym(n), opp(n), sym_opp(n))
        for n in range(3**STENCIL_SIZE)
        if n != sym_opp(n)
    }
)

n = len(indices)
print(f"const STENCIL_INDICES: [usize; {n}] = {indices};")
print(f"const STENCIL_INDICES_SYM: [usize; {n}] = {list(map(sym, indices))};")
print(f"const STENCIL_INDICES_OPP: [usize; {n}] = {list(map(opp, indices))};")
print(f"const STENCIL_INDICES_SYM_OPP: [usize; {n}] = {list(map(sym_opp, indices))};")
