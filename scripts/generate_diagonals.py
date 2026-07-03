import sys


def print_lines(diag_name, lines):
    print("#[rustfmt::skip]")
    print(
        f"pub const {diag_name.upper()}WARD_DIAGONALS: [&[(usize, usize)]; 2 * BOARD_SIZE - 1] = ["
    )
    print("\n".join(f"    &{line}," for line in lines))
    print("];")


def upward(board_size):
    lines = []
    for y in reversed(range(1, board_size)):
        line = [(x, x + y) for x in range(board_size - y)]
        lines.append(line)
    for x in range(board_size):
        line = [(x + y, y) for y in range(board_size - x)]
        lines.append(line)
    print_lines("up", lines)


def downward(board_size):
    lines = []
    for y in reversed(range(1, board_size)):
        line = [(x, board_size - y - x - 1) for x in range(board_size - y)]
        lines.append(line)
    for x in range(board_size):
        line = [(x + y, board_size - y - 1) for y in range(board_size - x)]
        lines.append(line)
    print_lines("down", lines)


def main():
    assert len(sys.argv) == 2, f"Usage: {sys.argv[0]} <board_size>"
    board_size = int(sys.argv[1])
    assert board_size > 0
    upward(board_size)
    print()
    downward(board_size)


if __name__ == "__main__":
    main()
