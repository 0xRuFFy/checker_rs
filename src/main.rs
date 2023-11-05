use checkers_core as core;

fn main() {
    println!("Hello, world!");

    let board = core::Board::new();

    // println!("{}", board.to_string(board.get_white_possible_moves()));
    println!("{}", board);

    println!("{:?}", board.get_white_possible_moves());

    // println!("{}", board);
}
