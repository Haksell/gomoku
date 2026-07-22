# gomoku

## todo

### misc

- bitboard: min of 4 rotations, 2 flips, and white=-black
- simplify coeffistic and training:
  - put unique coeffs before stencil coeffs to simplify indexing
  - create proper `Coeffs` type instead of jumbled array
- update double threes in `do_move` and `undo_move`
- dynamic heuristic update on `do_move` (each position updates predetermined amount of states BITBOARD)
- wins by captures or alignment in stats
- unit test of rules
- compute during human time (flag)
- check rust performance book for better compilation
- fix buggy open threes `w.xxx.o`
- feature nannou: don't compile the lib if running headless
- remove most `usize`s
- lazy update legal moves for both black and white
- transposition table for iterative deepening
- remove most allocations
- coup force (4 captures...)
- clock in human vs (human | bot)
- no depth in leaf_value, if win found in iterative deepening, stop
- futility pruning
- more depth -> smaller radius
- better multithreading (rayon -> manual pool?)
- include border in stencil (train on 13x13 board, otherwise pretty much irrelevant)
- `mod bots` -> `mod search`

### self-improving heuristic

- on calcule l'heuristique a depth 1/2
- on fait l'alpha-beta pruning a depth 3/4 (il faut la meme parite qu'a la premiere etape pour que ca soit biaise pour le meme joueur)
- on compare
- si l'heuristique a depth 3/4 est plus grande on incremente la valeur des patterns a depth 1/2, sinon on decremente

### patterns with forbidden moves

Il faudrait 6 types de cases :

- noir
- blanc
- vide jouable par les deux
- vide jouable par les noirs
- vide jouable par les blancs
- vide jouable par personne

### design

- highlight close moves while bot is thinking
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
- centrality:
  - sum of stone distances to center was tried and gave poor results
  - try distance to edges?
  - try distance to corners?

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
- [ ] SSS\*
- [ ] MCTS
- [ ] MCTS solver for endgame (different concept than MCTS)
- [ ] ƎUИИ
- [ ] Beam Search

### bonus ideas

- [x] bot vs bot
- [ ] alternate start rules (swap, swap2...)
- [ ] variable board size (clap arg?)
- [ ] gomoku vs renju rules
- [ ] cancel move:
  - [ ] left-right keys to rewind the game like on lichess
  - [ ] backspace to reset the game to last human move
- [ ] ratatui version
- [ ] mobile version
- [ ] web version

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
- [x] If your AI takes more than half a second (in average) to find a move, you will not validate the project.
- [ ] You will not get all the points if your implementation wins too slowly (in too many moves).
- [ ] You will not get all the points if your implementation has low search depth.
- [x] You will not get all the points if your implementation has a naive implementation.

### mandatory part

- [x] The executable must be named `Gomoku`. `ln -sf target/release/gomoku Gomoku`
- [ ] Human vs AI. The goal here is that the program wins the game, without you letting it win. It must be able to adapt its strategy to the player's moves.
- [ ] Human vs Human, with a move suggestion feature.
- [x] You have to use a Minimax algorithm, or a variant.
- [x] You need an efficient heuristic function to evaluate the value of a terminal node in your tree.
- [x] You must also provide a usable graphical interface to allow one to actually play Gomoku.
- [ ] Implement some sort of debugging process that lets you examine the reasoning process of your AI while it's running.
- [ ] You have to display somewhere in your user interface a timer that counts how much time your AI takes to find its next move.

## push check

- ctrl+f `todo`
- ctrl+f `unimplemented`
- remove old/new files
- if some lib is not used seriously (e.g. `itertools` or `indicatif`), remove the dependency
- avoid `panic`s and `assert`s (`eprintln` + `exit(1 | 2)`)

## after push

- remove useless `Makefile` rules
- clean this `README.md`

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
