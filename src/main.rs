#![allow(dead_code)]

mod core;

use core::Board;

fn main() {
    // println!("Hello, world!");

    let mut board = Board::new();

    println!("{}", board);

    board.move_piece(16, 25);

    println!("{}", board);
}
