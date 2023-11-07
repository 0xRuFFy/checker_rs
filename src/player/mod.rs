mod alpha_beta_player;
mod alpha_zero_player;
pub mod bot_player;
mod human_player;
mod minimax_player;
mod monte_carlo_player;

pub use bot_player::BotPlayer;
pub use human_player::HumanPlayer;
pub use minimax_player::MinimaxPlayer;

use checkers_core as core;

pub trait Player {
    fn init(&mut self, color: core::board::PieceColor);
    fn get_color(&self) -> core::board::PieceColor;
    fn swap_color(&mut self) -> core::board::PieceColor;
    fn get_move(&self, board: &core::Board, possible_moves: &Vec<(u8, Vec<u8>)>) -> (u8, u8);
}
