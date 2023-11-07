use checkers_core as core;

use super::Player;

pub struct HumanPlayer {
    color: core::board::PieceColor,
}

impl HumanPlayer {
    pub fn new() -> HumanPlayer {
        return HumanPlayer {
            color: core::board::WHITE,
        };
    }
}

impl Player for HumanPlayer {
    fn init(&mut self, color: core::board::PieceColor) {
        self.color = color;
    }

    // Note: This function does not need to be efficient since it is only called once and won't be called in in for example a Monte Carlo Tree Search / Minimax algorithm
    fn get_move(&self, board: &core::Board, possible_moves: &Vec<(u8, Vec<u8>)>) -> (u8, u8) {
        let moves_count = possible_moves.len();
        let mut input = String::new();
        let from: u8;
        let to: u8;
        loop {
            println!("Select piece: ");
            input.clear();
            std::io::stdin().read_line(&mut input).unwrap();
            let mut iter = input.split_whitespace();
            if let Some(from_str) = iter.next() {
                if let Ok(from_int) = from_str.parse::<u8>() {
                    if (from_int as usize) < moves_count {
                        from = from_int;
                        break;
                    }
                }
            }
        }
        loop {
            println!("Select move: ");
            input.clear();
            std::io::stdin().read_line(&mut input).unwrap();
            let mut iter = input.split_whitespace();
            if let Some(to_str) = iter.next() {
                if let Ok(to_int) = to_str.parse::<u8>() {
                    if to_int < possible_moves[from as usize].1.len() as u8 {
                        to = to_int;
                        break;
                    }
                }
            }
        }
        return (
            possible_moves[from as usize].0,
            possible_moves[from as usize].1[to as usize],
        );
    }

    fn get_color(&self) -> core::board::PieceColor {
        return self.color;
    }

    fn swap_color(&mut self) -> core::board::PieceColor {
        self.color = !self.color;
        return self.color;
    }
}
