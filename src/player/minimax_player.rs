use super::Player;
use crate::{game::GameState, player::BotPlayer};
use checkers_core as core;
use core::board::PieceColor;

pub struct MinimaxPlayer {
    color: core::board::PieceColor,
}

impl MinimaxPlayer {
    pub fn new() -> Self {
        MinimaxPlayer {
            color: core::board::WHITE,
        }
    }

    fn minimax(
        &self,
        board: &core::Board,
        depth: u8,
        maximizing_player: bool,
    ) -> (Option<(u8, u8)>, i32) {
        if depth == 0 {
            return (None, BotPlayer::eval_v1(board, self.color));
        }

        match BotPlayer::get_game_state(board) {
            GameState::Winner(color) => {
                if color == self.color {
                    return (None, i32::MAX);
                } else {
                    return (None, i32::MIN);
                }
            }
            GameState::InProgress => {}
        }

        let mut best_value = if maximizing_player {
            i32::MIN
        } else {
            i32::MAX
        };

        let moves = board.get_possible_moves(&self.color);
        let mut best_move = None;
        for (from, tos) in moves {
            for to in tos {
                let mut new_board = board.clone();
                new_board.move_piece(from, to);

                let (option_move, value) = self.minimax(&new_board, depth - 1, !maximizing_player);
                if maximizing_player {
                    best_value = best_value.max(value);
                } else {
                    best_value = best_value.min(value);
                }
                if best_value == value {
                    best_move = Some((from, to));
                }
            }
        }

        return (best_move, best_value);
    }
}

impl Player for MinimaxPlayer {
    fn init(&mut self, color: core::board::PieceColor) {
        self.color = color;
    }

    fn get_color(&self) -> core::board::PieceColor {
        return self.color;
    }

    fn swap_color(&mut self) -> core::board::PieceColor {
        self.color = !self.color;
        return self.color;
    }

    fn get_move(&self, board: &core::Board) -> (u8, u8) {
        let (move_option, _) = self.minimax(board, 3, true);
        return move_option.unwrap();
    }
}
