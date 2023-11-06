mod human_player;

pub use human_player::HumanPlayer;

use checkers_core as core;

pub trait Player {
    fn init(&mut self, color: core::board::PieceColor);
    fn get_color(&self) -> core::board::PieceColor;
    fn swap_color(&mut self) -> core::board::PieceColor;
    fn get_move(&self, board: &core::Board) -> (u8, u8);
}
