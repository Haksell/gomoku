# gomoku

## general guidelines

- [ ] Two players take turns placing stones of their color on an intersection of the board, and the game ends when one player manages to align five stones.
- [ ] Sometimes, only an alignment of 5 can win, and sometimes 5 or more is okay. In the context of this projet, we will consider 5 or more to be a win.
- [ ] There are different interpretations on what the board size for Gomoku should be. In the context of this project, Gomoku will be played on a 19x19 Goban, without limit to the number of stones.
- [ ] There are a great many additional rules to Gomoku (Google it!) aimed at making the game more fair (regular Gomoku is proven to be unfair, a perfect first player wins 100% of the time) and more interesting.
- [ ] Capture (As in the Ninuki-renju or Pente variants) : You can remove a pair of your opponent’s stones from the board by flanking them with your own stones (See the appendix). This rule adds a win condition : If you manage to capture ten of your opponent’s stones, you win the game.
- [ ] A player who manages to line up five stones wins only if the opponent cannot break this line by capturing a pair.
- [ ] If the player has already lost four pairs and the opponent can capture one more, the opponent wins by capture.
- [ ] If there is no possibility of this happening, there is no need to continue the game.
- [ ] No double-threes : It is forbidden to play a move that introduces two free-three alignments, which would guarantee a win by alignment (See the appendix). Gomoku Yeah, well, your brain has to fry sometime
- [ ] You are free to use whatever language and graphical interface library you want.
- [ ] Your program should not crash in any circumstances (even when it runs out of memory), and should not quit unexpectedly. If it happens, your project will be considered non-functional and your grade will be 0.
- [ ] You have to provide a Makefile which must produce your program. It must not relink.
- [ ] Your Makefile must at least contain the rules: $(NAME), all, clean, fclean and re.

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
