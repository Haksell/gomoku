use crate::{
    Args,
    game::{Game, GameState},
    player::PlayerColor,
};

pub fn run(args: &Args) {
    assert!(
        args.black_player.is_bot() || args.white_player.is_bot(),
        "`num_games` is reserved for bot vs bot matches."
    );

    let mut black_wins = 0.;
    for game_idx in 1..=args.num_games {
        let mut game = Game::new(args.black_player, args.white_player);
        game.play_game();
        match game.state {
            GameState::Playing => unreachable!(),
            GameState::Won(PlayerColor::Black) => black_wins += 1.,
            GameState::Draw => black_wins += 0.5,
            GameState::Won(PlayerColor::White) => {}
        }
        let black_percentage = 100. * black_wins / game_idx as f64;
        println!("Black won {black_wins}/{game_idx} games ({black_percentage:.1}%)");
    }
}
