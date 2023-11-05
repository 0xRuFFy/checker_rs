use checkers_core as core;

fn main() {
    println!("Hello, world!");

    let mut board = core::Board::new();

    println!("{}", board);

    println!("{:?}", board.get_possible_moves_of(16));
    // for i in board.get_possible_moves_of(2).unwrap() {
    //     println!("{}", core::Board::is_valid_id(i));
    // }

    println!("{:?}", board.get_white_possible_moves());
        
    // println!("{}", board);
}
