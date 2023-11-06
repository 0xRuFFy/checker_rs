use checkers_core as core;

use super::Player;

pub struct AlphaZeroPlayer {
    color: core::board::PieceColor,
}

impl AlphaZeroPlayer {
    pub fn new() -> Self {
        AlphaZeroPlayer {
            color: core::board::WHITE,
        }
    }
}

impl Player for AlphaZeroPlayer {
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
