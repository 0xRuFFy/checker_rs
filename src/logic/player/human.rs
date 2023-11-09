use super::{Board, Player, PossibleMoves};
use std::io::*;

pub struct HumanPlayer {
    color: bool,
}

impl HumanPlayer {
    pub fn new() -> Self {
        Self { color: false }
    }
}

impl Player for HumanPlayer {
    fn init(&mut self, color: bool) {
        self.color = color;
    }

    fn get_move(&mut self, board: &Board, possible_moves: &Vec<PossibleMoves>) -> (u8, u8) {
        println!("{:?}", possible_moves);
        let mut input = String::new();
        let from: u8;
        let to: u8;

        loop {
            print!("Select a piece to move [Index]: ");
            stdout().flush().unwrap();
            input.clear();
            stdin().read_line(&mut input).unwrap();
            match input.trim().parse::<u8>() {
                Ok(n) => {
                    if (n as usize) >= possible_moves.len() {
                        println!("Invalid input!");
                        continue;
                    };
                    from = n;
                    break;
                }
                Err(_) => {
                    println!("Invalid input!");
                }
            };
        }

        loop {
            print!("Select a destination [Index]: ");
            stdout().flush().unwrap();
            input.clear();
            stdin().read_line(&mut input).unwrap();
            match input.trim().parse::<u8>() {
                Ok(n) => {
                    if (n as usize) >= possible_moves[from as usize].to.len() {
                        println!("Invalid input!");
                        continue;
                    };
                    to = n;
                    break;
                }
                Err(_) => {
                    println!("Invalid input!");
                }
            };
        }

        return (
            possible_moves[from as usize].from,
            possible_moves[from as usize].to[to as usize],
        );
    }
}
