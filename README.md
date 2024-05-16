# gomoku

## general guidelines

- [ ] Two players take turns placing stones of their color on an intersection of the board
- [x] The game ends when one player manages to align 5 stones or more.
- [ ] Gomoku will be played on a 19x19 Goban, without limit to the number of stones.
- [ ] You can capture a pair of your opponent's stones from the board by flanking them with your own stones.
- [ ] A player cannot move into a capture.
- [ ] If you manage to capture 5 pairs of your opponent's stones, you win the game.
- [ ] A player who manages to line up five stones wins only if the opponent cannot break this line by capturing a pair.
- [ ] If the player has already lost four pairs and the opponent can capture one more, the opponent wins by capture.
- [ ] If there is no possibility of this happening, there is no need to continue the game.
- [ ] It is forbidden to play a move that introduces two free-three alignments, which would guarantee a win by alignment.
- [ ] it is not forbidden to introduce a double-three by capturing a pair.
- [ ] Your program should not crash in any circumstances (even when it runs out of memory), and should not quit unexpectedly. (no `unwrap`...)
- [ ] You have to provide a Makefile which must produce your program. It must not relink.
- [ ] Your Makefile must at least contain the rules: $(NAME), all, clean, fclean and re.
- [ ] If your AI takes more than half a second (in average) to find a move, you will not validate the project.
- [ ] You will not get all the points if your implementation wins too slowly (in too many moves).
- [ ] You will not get all the points if your implementation has low search depth.
- [ ] You will not get all the points if your implementation has a naive implementation.

## mandatory part

- [ ] The executable must be named Gomoku.
- [ ] Human vs AI. The goal here is that the program wins the game, without you letting it win. It must be able to adapt its strategy to the player’s moves.
- [ ] Human vs Human, with a move suggestion feature.
- [ ] You have to use a Minimax algorithm, or a variant.
- [ ] You need an efficient heuristic function to evaluate the value of a terminal node in your tree.
- [ ] You must also provide a usable graphical interface to allow one to actually play Gomoku.
- [ ] Implement some sort of debugging process that lets you examine the reasoning process of your AI while it’s running.
- [ ] You have to display somewhere in your user interface a timer that counts how much time your AI takes to find its next move.

## bonus ideas

- [ ] alternate rules
- [ ] cancel move
- [ ] bot arena
- [ ] mobile version
- [ ] web version

## todo

- basic ui with correct rules
- place the pieces on intersections
- tooltip to show the player piece with some transparency
- draw 9 dots
- unit test of rules
- random bot
- bot that likes center
- arena with elo ranking
- draw notation
- make the default ui as least as pretty as the one from alabalet
- different themes

## libraries

- https://github.com/nannou-org/nannou (home + 42)
- https://github.com/DioxusLabs/dioxus (home)
- https://github.com/tauri-apps/tauri
- https://github.com/bevyengine/bevy
- https://github.com/cunarist/rinf
- https://github.com/iced-rs/iced
- https://github.com/Relm4/Relm4
- https://github.com/emilk/egui (home + 42)

## resources

- https://www.chessprogramming.org/Iterative_Deepening
- https://www.chessprogramming.org/Transposition_Table
- https://stackoverflow.com/questions/41756443/how-to-implement-iterative-deepening-with-alpha-beta-pruning
- https://en.wikipedia.org/wiki/Zobrist_hashing
- https://www.chessprogramming.org/ProbCut
- https://wiki.cs.pdx.edu/cs542-spring2013/papers/buro/probcut.pdf