use crate::game::GameState;

use super::{
    alpha_zero_player::AlphaZeroPlayer,
    minimax_player::MinimaxPlayer, monte_carlo_player::MonteCarloPlayer,
};
use checkers_core as core;

#[derive(Clone, Copy)]
pub enum Depth {
    Static(u8),
}

pub struct BotPlayer;

impl BotPlayer {
    pub fn minimax(depth: Depth, eval_version: u8) -> MinimaxPlayer {
        return MinimaxPlayer::new(&depth, eval_version);
    }

    pub fn monte_carlo() -> MonteCarloPlayer {
        todo!()
    }

    pub fn alpha_zero() -> AlphaZeroPlayer {
        todo!()
    }

    pub fn get_game_state(board: &core::Board) -> GameState {
        if board.white_count() == 0 {
            return GameState::Winner(core::board::BLACK);
        }
        if board.black_count() == 0 {
            return GameState::Winner(core::board::WHITE);
        }
        // TODO: Check for draw
        return GameState::InProgress;
    }
}
