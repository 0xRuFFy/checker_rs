mod logic;
mod util;

#[allow(unused_imports)]
use crate::logic::{eval, HumanPlayer, MinimaxPlayer};
use logic::Game;
use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    /// The depth of the minimax algorithm
    #[arg(short, long, default_value_t = 7)]
    depth: u8,
    /// The evaluation function to use [version 1, 2...]
    #[arg(short, long, default_value_t = 2)]
    eval: u8,
}

fn main() {
    let args = Cli::parse();

    // let mut game = Game::new(Box::new(HumanPlayer::new()), Box::new(HumanPlayer::new()));
    let mut game = Game::new(
        Box::new(MinimaxPlayer::new(args.depth, match args.eval {
            1 => eval::v1,
            2 => eval::v2,
            _ => panic!("Invalid evaluation function"),
        })),
        Box::new(HumanPlayer::new()),
    );
    game.play();

    // let mut board = Board::new();
    // println!("{}", board);
    // println!("{:?}", board.possible_moves(piece::WHITE));
    // // println!("{:?}", board.possible_jumps_from(18));

    // let move_info = board.move_piece(18, 25);
    // println!("{:?}", move_info);
    // println!("{}", board);
}
