use crate::{
    Args,
    game::{Game, GameState},
    player::PlayerColor,
};

// TODO random initial position
pub fn run(args: &Args) {
    assert!(
        args.black_player.is_bot() || args.white_player.is_bot(),
        "`num_games` is reserved for bot vs bot matches."
    );

    let mut player1_wins = 0.;
    for game_idx in 1..=args.num_games {
        let (player1_color, black, white) = match game_idx & 1 {
            0 => (PlayerColor::White, args.white_player, args.black_player),
            1 => (PlayerColor::Black, args.black_player, args.white_player),
            _ => unreachable!(),
        };

        let mut game = Game::new(black, white);
        // TODO: flags to configure random board (n_moves and dist_to_center)
        game.play_random_moves(4, 5);
        game.play_game();

        match game.state {
            GameState::Playing => unreachable!(),
            GameState::Draw => player1_wins += 0.5,
            GameState::Won(winner) if winner == player1_color => player1_wins += 1.,
            GameState::Won(_) => {}
        }

        let player1_percentage = 100. * player1_wins / game_idx as f64;
        println!("Player 1 won {player1_wins}/{game_idx} games ({player1_percentage:.1}%)");
    }
}
