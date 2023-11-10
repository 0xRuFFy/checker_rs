pub mod eval;
mod human;
mod minimax;

pub use eval::{v1, v2};
pub use human::HumanPlayer;
pub use minimax::MinimaxPlayer;

use crate::board::{Board, PossibleMoves};

pub trait Player {
    fn init(&mut self, color: bool);
    fn get_move(&mut self, board: &Board, possible_moves: &[PossibleMoves]) -> (u8, u8);
}
