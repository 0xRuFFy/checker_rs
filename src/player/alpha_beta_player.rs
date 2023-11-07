use checkers_core as core;

use super::Player;

pub struct AlphaBetaPlayer {
    color: core::board::PieceColor,
}

impl AlphaBetaPlayer {
    pub fn new() -> Self {
        AlphaBetaPlayer {
            color: core::board::WHITE,
        }
    }
}

impl Player for AlphaBetaPlayer {
    fn init(&mut self, color: core::board::PieceColor) {
        todo!()
    }

    fn get_color(&self) -> core::board::PieceColor {
        todo!()
    }

    fn swap_color(&mut self) -> core::board::PieceColor {
        todo!()
    }

    fn get_move(&mut self, board: &core::Board, possible_moves: &Vec<(u8, Vec<u8>)>) -> (u8, u8) {
        todo!()
    }
}
