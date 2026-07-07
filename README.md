# gomoku

## todo

### misc

- dynamic heuristic update (each position updates predetermined amount of states BITBOARD)
- wins by captures or alignment in stats
- unit test of rules
- compute during human time (flag)
- iterative alpha-beta pruning to always return a move in less than 1s
- arena with elo ranking
- check rust performance book for better compilation
- fix buggy open threes `w.xxx.o`
- feature nannou: don't compile the lib if running headless arena
- remove most `usize`s
- lazy update legal moves for both black and white
- transposition table for iterative deepening
- remove most allocations
- idabp: truncate based on cached heuristic
- coup force (4 captures...)
- clock in human vs (human | bot)
- no depth in leaf_value, if win found in iterative deepening, stop

### design

- change hover font
- show ply
- prefer double wins (captures + alignment)
- show last capture (screen shake)
- show number of captures
- fix move shows as hover before bot plays (async bot)
- multiple themes
- confetti
- eval bar
- blinking capture counter on win by capture
- bowling alley animations

### heuristics

- optimize constants through SPSA or simulated annealing
- more patterns
- double attacks (multiply 4 dp en gros)
- smarter dist to center:
  - dist to edge?
  - dist to corner?
  - manhattan or something else?

### flopped heuristics

- locked 3: `wbbbw`
- locked 3 of 4: `wbbb.w` | `wbb.bw` | `wb.bbw` | `w.bbbw`

### algorithms

- [x] Random
- [x] Minimax
- [x] Negamax
- [x] Alpha-Beta Pruning
- [x] Iterative Deepening Alpha-Beta Pruning
- [ ] Principal Variation Search
- [ ] Quiescence Search
- [ ] ProbCut
- [ ] Multi-Cut
- [ ] Null Move Pruning
- [ ] NegaScout
- [ ] MTD(f)
- [ ] SSS*
- [ ] MCTS
- [ ] MCTS solver for endgame (different concept than MCTS)
- [ ] ƎUИИ
- [ ] Beam Search

### bonus ideas

- [x] bot vs bot
- [ ] bot arena
- [ ] alternate start rules (swap, swap2...)
- [ ] variable board size (clap arg?)
- [ ] gomoku vs renju rules
- [ ] cancel move
- [ ] ratatui version
- [ ] mobile version
- [ ] web version
- [ ] board size

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
- [x] If the player has already lost four pairs and the opponent can capture one more, the opponent wins by capture.

### general guidelines

- [x] Your program should not crash in any circumstances (even when it runs out of memory), and should not quit unexpectedly. (no `unwrap`...)
- [x] You have to provide a Makefile which must produce your program. It must not relink.
- [x] Your Makefile must at least contain the rules: $(NAME), all, clean, fclean and re.
- [ ] If your AI takes more than half a second (in average) to find a move, you will not validate the project.
- [ ] You will not get all the points if your implementation wins too slowly (in too many moves).
- [ ] You will not get all the points if your implementation has low search depth.
- [ ] You will not get all the points if your implementation has a naive implementation.

### mandatory part

- [x] The executable must be named `Gomoku`. `ln -sf target/release/gomoku Gomoku`
- [ ] Human vs AI. The goal here is that the program wins the game, without you letting it win. It must be able to adapt its strategy to the player's moves.
- [ ] Human vs Human, with a move suggestion feature.
- [x] You have to use a Minimax algorithm, or a variant.
- [ ] You need an efficient heuristic function to evaluate the value of a terminal node in your tree.
- [x] You must also provide a usable graphical interface to allow one to actually play Gomoku.
- [ ] Implement some sort of debugging process that lets you examine the reasoning process of your AI while it's running.
- [ ] You have to display somewhere in your user interface a timer that counts how much time your AI takes to find its next move.

## push check

- ctrl+f `todo`
- ctrl+f `unimplemented`
- `rm acoph`
- src/heuristics/old.rs + src/heuristics/new.rs -> src/heuristics/heuristic.rs
- if some lib is not used seriously (e.g. itertools), remove the dependency
- avoid panics and asserts (eprintln + exit(1 | 2))

## evaluation

### preliminary checks

- [ ] there is something in the git repository
- [ ] the Makefile is present and has the required rules
- [ ] no crash

### interface

- [ ] rules are implemented properly
- [ ] human-vs-human is playable
- [ ] human-vs-bot is playable

### algorithm

- [ ] takes less than half a second on average
- [ ] there is a timer indicating how much time the AI takes
- [ ] performance (AI victory in under 20 moves -> 5)
- [ ] implementation (alpha-beta/negascout/mtdf -> 5)
- [ ] search depth (10 or more levels -> 5)
- [ ] search space (multiple rectangular windows emcompassing placed stones but minimizing wasted space -> 5)

### heuristics

- [ ] does the heuristic take current alignments into account?
- [ ] does the heuristic check whether an alignment has enough space to develop into a 5-in-a-row?
- [ ] does the heuristic weigh an alignment according to its freedom (free, half-free, flanked)?
- [ ] does the heuristic take potential captures into account?
- [ ] does the heuristic take current captured stones into account?
- [ ] does the heuristic check for advanteageous combinations?
- [ ] does the heuristic take both players into account?
- [ ] does the heuristic take past player actions into account to identify patterns and weigh board states accordingly?
