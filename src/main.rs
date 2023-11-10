mod logic;
mod util;

use crate::logic::Board;
#[allow(unused_imports)]
use crate::logic::{eval, HumanPlayer, MinimaxPlayer};
use clap::Parser;
use logic::Game;

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
    let board = Board::from_fen("8/8/8/8/8/8/8/M1M1M1M1");
    if let Ok(board) = board {
        println!("{}", board);
    } else if let Err(e) = board {
        println!("{}", e);
    }

    let args = Cli::parse();

    // let mut game = Game::new(Box::new(HumanPlayer::new()), Box::new(HumanPlayer::new()));
    let mut game = Game::new(
        Box::new(MinimaxPlayer::new(
            args.depth,
            match args.eval {
                1 => eval::v1,
                2 => eval::v2,
                _ => panic!("Invalid evaluation function"),
            },
        )),
        Box::new(HumanPlayer::new()),
    );
    game.play();
}
