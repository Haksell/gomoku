import sys


assert len(sys.argv) == 2, f"Usage: {sys.argv[0]} <STENCIL_SIZE>"
STENCIL_SIZE = int(sys.argv[1])


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
print(f"pub static STENCIL_INDICES: [usize; UNIQUE_STENCIL_INDICES] = {indices};")
print(
    f"pub static STENCIL_INDICES_SYM: [usize; UNIQUE_STENCIL_INDICES] = {list(map(sym, indices))};"
)
print(
    f"pub static STENCIL_INDICES_OPP: [usize; UNIQUE_STENCIL_INDICES] = {list(map(opp, indices))};"
)
print(
    f"pub static STENCIL_INDICES_SYM_OPP: [usize; UNIQUE_STENCIL_INDICES] = {list(map(sym_opp, indices))};"
)
