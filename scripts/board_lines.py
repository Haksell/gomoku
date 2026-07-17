import sys


def rows(board_size):
    return [[(x, y) for x in range(board_size)] for y in range(board_size)]


def columns(board_size):
    return [[(y, x) for x in range(board_size)] for y in range(board_size)]


def upward(board_size):
    lines = []
    for y in reversed(range(1, board_size)):
        line = [(x, x + y) for x in range(board_size - y)]
        lines.append(line)
    for x in range(board_size):
        line = [(x + y, y) for y in range(board_size - x)]
        lines.append(line)
    return lines


def downward(board_size):
    lines = []
    for y in reversed(range(1, board_size)):
        line = [(x, board_size - y - x - 1) for x in range(board_size - y)]
        lines.append(line)
    for x in range(board_size):
        line = [(x + y, board_size - y - 1) for y in range(board_size - x)]
        lines.append(line)
    return lines


def main():
    assert len(sys.argv) == 3, f"Usage: {sys.argv[0]} <board_size> <stencil_size>"
    board_size = int(sys.argv[1])
    stencil_size = int(sys.argv[2])
    assert board_size > 0
    assert stencil_size > 0
    assert stencil_size <= board_size
    lines = [
        line
        for f in [rows, columns, upward, downward]
        for line in f(board_size)
        if len(line) >= stencil_size
    ]
    print("#[rustfmt::skip]")
    print("pub const LINES: [&[Position]; 6 * BOARD_SIZE - 4 * STENCIL_SIZE + 2] = [")
    print("\n".join(f"    &{line}," for line in lines))
    print("];")


if __name__ == "__main__":
    main()
