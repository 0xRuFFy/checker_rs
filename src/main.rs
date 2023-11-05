use checkers_core::Board;

fn main() {
    println!("Hello, world!");

    let mut board = Board::new();

    println!("{}", board);

    println!("{:?}", board.get_possible_moves_of(2));
    for i in board.get_possible_moves_of(2).unwrap() {
        println!("{}", Board::is_valid_id(i));
    }

    // println!("{}", board);
}
