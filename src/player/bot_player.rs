use crate::game::GameState;

use super::{
    alpha_beta_player::AlphaBetaPlayer, alpha_zero_player::AlphaZeroPlayer,
    minimax_player::MinimaxPlayer, monte_carlo_player::MonteCarloPlayer,
};
use checkers_core as core;
use core::board::PieceColor;

pub struct BotPlayer;

impl BotPlayer {
    pub fn minimax() -> MinimaxPlayer {
        MinimaxPlayer::new()
    }

    pub fn alpha_beta() -> AlphaBetaPlayer {
        todo!()
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

    pub fn eval_v1(board: &core::Board, color: PieceColor) -> i32 {
        let mut score = 0;
        for i in 0..8 {
            for j in 0..8 {
                if let Some((piece_color, _)) = board.get_piece_by_coords(i, j) {
                    if piece_color == color {
                        score += 1;
                    } else {
                        score -= 1;
                    }
                }
            }
        }
        return score;
    }
}
