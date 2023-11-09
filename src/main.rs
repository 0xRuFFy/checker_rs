mod logic;
mod util;

use logic::Game;

use crate::logic::{eval, HumanPlayer, MinimaxPlayer};

fn main() {
    println!("Hello, world!");

    // let mut game = Game::new(Box::new(HumanPlayer::new()), Box::new(HumanPlayer::new()));
    let mut game = Game::new(
        Box::new(MinimaxPlayer::new(9, eval::v2)),
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
