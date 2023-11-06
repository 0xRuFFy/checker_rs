use checkers_core as core;

use super::Player;

pub struct MonteCarloPlayer {
    color: core::board::PieceColor,
}

impl MonteCarloPlayer {
    pub fn new() -> Self {
        MonteCarloPlayer {
            color: core::board::WHITE,
        }
    }
}

impl Player for MonteCarloPlayer {
    fn init(&mut self, color: core::board::PieceColor) {
        todo!()
    }

    fn get_color(&self) -> core::board::PieceColor {
        todo!()
    }

    fn swap_color(&mut self) -> core::board::PieceColor {
        todo!()
    }

    fn get_move(&self, board: &core::Board) -> (u8, u8) {
        todo!()
    }
}
