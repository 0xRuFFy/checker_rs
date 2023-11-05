mod core;

use core::Board;

fn main() {
    println!("Hello, world!");

    let board = Board::new();

    println!("{}", board);
}
