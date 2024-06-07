# gomoku

## todo

### misc

- fast `undo_move`
- decent heuristic
- implement draws
- 1v1 bot to estimate level
- bot arena
- force capturing pair if five in a row
- finish implementing rules
- handle draws
- unit test of rules
- random bot
- bot that likes center
- arena with elo ranking
- `Vec2<f32>` (physical pos), `Vec2<usize>` (board pos) and `Vec2<isize>` (direction)
- make the code compile faster
- implement forced draws?
- compute during human time?
- iterative alpha-beta pruning to always return a move in less than 1s

### design

- show coordinates of mouse on hover
- show number of captures
- show winner (or draw)
- stone shadow
- hover transparency
- multiple themes

## subject

### rules

- [x] Two players take turns placing stones of their color on an intersection of the board
- [x] The game ends when one player manages to align 5 stones or more.
- [x] Gomoku will be played on a 19x19 Goban, without limit to the number of stones.
- [x] You can capture a pair of your opponent's stones from the board by flanking them with your own stones.
- [x] If you manage to capture 5 pairs of your opponent's stones, you win the game.
- [x] It is forbidden to play a move that introduces 2 free-three alignments, which would guarantee a win by alignment.
- [x] It is not forbidden to introduce a double-three by capturing a pair.
- [x] A player who manages to line up five stones wins only if the opponent cannot break this line by capturing a pair.
- [x] If the player has already lost four pairs and the opponent can capture one more, the opponent wins by capture. (???)
- [x] If there is no possibility of this happening, there is no need to continue the game. (???)

### general guidelines

- [ ] Your program should not crash in any circumstances (even when it runs out of memory), and should not quit unexpectedly. (no `unwrap`...)
- [ ] You have to provide a Makefile which must produce your program. It must not relink.
- [ ] Your Makefile must at least contain the rules: $(NAME), all, clean, fclean and re.
- [ ] If your AI takes more than half a second (in average) to find a move, you will not validate the project.
- [ ] You will not get all the points if your implementation wins too slowly (in too many moves).
- [ ] You will not get all the points if your implementation has low search depth.
- [ ] You will not get all the points if your implementation has a naive implementation.

### mandatory part

- [ ] The executable must be named `Gomoku`. `ln -s target/release/gomoku Gomoku`
- [ ] Human vs AI. The goal here is that the program wins the game, without you letting it win. It must be able to adapt its strategy to the player’s moves.
- [ ] Human vs Human, with a move suggestion feature.
- [ ] You have to use a Minimax algorithm, or a variant.
- [ ] You need an efficient heuristic function to evaluate the value of a terminal node in your tree.
- [ ] You must also provide a usable graphical interface to allow one to actually play Gomoku.
- [ ] Implement some sort of debugging process that lets you examine the reasoning process of your AI while it’s running.
- [ ] You have to display somewhere in your user interface a timer that counts how much time your AI takes to find its next move.

### bonus ideas

- [ ] alternate rules
- [ ] cancel move
- [ ] bot arena
- [ ] mobile version
- [ ] web version

## evaluation

- [ ] preliminary checks (git repository not empty, Makefile, no crash)
- [x] rules
- [ ] human-vs-human
- [ ] human-vs-bot
- [ ] takes less than half a second on average
- [ ] there is a timer indicating how much time the AI takes
- [ ] performance (AI victory in under 20 moves -> 5)
- [x] implementation (alpha-beta/negascout/mtdf -> 5)
- [ ] search depth (10 or more levels -> 5)
- [x] search space (multiple rectangular windows emcompassing placed stones but minimizing wasted space -> 5)
- [ ] does the heuristic take current alignments into account?
- [ ] does the heuristic check whether an alignment has enough space to develop into a 5-in-a-row?
- [ ] does the heuristic weigh an alignment according to its freedom (free, half-free, flanked)?
- [ ] does the heuristic take potential captures into account?
- [ ] does the heuristic take current captured stones into account?
- [ ] does the heuristic check for advanteageous combinations?
- [ ] does the heuristic take both players into account?
- [ ] does the heuristic take past player actions into account to identify patterns and weigh board states accordingly?
- [ ] bonuses: 1 point per identifiable, separate bonus

## resources

- https://github.com/nannou-org/nannou
- https://github.com/emilk/egui
- https://www.chessprogramming.org/Iterative_Deepening
- https://www.chessprogramming.org/Transposition_Table
- https://stackoverflow.com/questions/41756443/how-to-implement-iterative-deepening-with-alpha-beta-pruning
- https://en.wikipedia.org/wiki/Zobrist_hashing
- https://www.chessprogramming.org/ProbCut
- https://wiki.cs.pdx.edu/cs542-spring2013/papers/buro/probcut.pdf
- https://en.wikipedia.org/wiki/Principal_variation_search
- https://en.wikipedia.org/wiki/MTD(f)
