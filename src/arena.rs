use crate::{Args, game::Game, player::PlayerColor};

pub fn run(args: &Args) {
    assert!(
        args.black_player.is_bot() || args.white_player.is_bot(),
        "`num_games` is reserved for bot vs bot matches."
    );
    let mut black_wins = 0;
    let mut white_wins = 0;
    for game in 1..=args.num_games {
        let mut model = Game::new(args.black_player, args.white_player);
        model.play_game();
        #[expect(clippy::unimplemented)]
        match model.winner {
            Some(PlayerColor::Black) => black_wins += 1,
            Some(PlayerColor::White) => white_wins += 1,
            None => unimplemented!("draws are broken"),
        }
        let black_percentage = 100. * black_wins as f64 / game as f64;
        let white_percentage = 100. - black_percentage;
        println!("Black won {black_wins} games ({black_percentage:.1}%)");
        println!("White won {white_wins} games ({white_percentage:.1}%)");
        println!();
    }
}
