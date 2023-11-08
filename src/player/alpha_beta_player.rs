use std::collections::HashMap;

use checkers_core as core;

use crate::game::GameState;

use super::{bot_player::Depth, BotPlayer, Player};

pub struct AlphaBetaPlayer {
    color: core::board::PieceColor,
    depth: Depth,
    eval_version: u8,
    transposition_table: HashMap<core::Board, f32>,
}

impl AlphaBetaPlayer {
    pub fn new(depth: Depth, eval_version: u8) -> Self {
        if ![1, 2].contains(&eval_version) {
            panic!("Invalid eval version");
        }
        return AlphaBetaPlayer {
            color: core::board::WHITE,
            depth,
            eval_version,
            transposition_table: HashMap::new(),
        };
    }

    fn alpha_beta(
        &mut self,
        board: &core::Board,
        depth: u8,
        maximizing_player: bool,
        mut alpha: f32,
        mut beta: f32,
    ) -> f32 {
        if let Some(value) = self.transposition_table.get(board) {
            return *value;
        }

        if depth == 0 {
            match self.eval_version {
                1 => return board.eval_v1(self.color),
                2 => return board.eval_v2(self.color),
                _ => panic!("Invalid eval version"),
            }
        }

        match BotPlayer::get_game_state(board) {
            GameState::Winner(color) => {
                if color == self.color {
                    return f32::INFINITY;
                } else {
                    return f32::NEG_INFINITY;
                }
            }
            GameState::InProgress => {}
        }

        let mut best_value = if maximizing_player {
            f32::NEG_INFINITY
        } else {
            f32::INFINITY
        };

        let color = if maximizing_player {
            self.color
        } else {
            !self.color
        };

        let moves = board.get_possible_moves(&color);
        for (from, tos) in moves {
            for to in tos {
                let mut new_board = board.clone();
                let jumped = new_board.move_piece(from, to);

                let value = self.alpha_beta(
                    &new_board,
                    depth - 1,
                    if jumped {
                        maximizing_player
                    } else {
                        !maximizing_player
                    },
                    alpha,
                    beta,
                );

                self.transposition_table.insert(new_board, value);
                if maximizing_player {
                    best_value = best_value.max(value);
                    alpha = alpha.max(value);
                } else {
                    best_value = best_value.min(value);
                    beta = beta.min(value);
                }

                if beta <= alpha {
                    break;
                }
            }
        }

        return best_value;
    }

    pub fn clear_transposition_table(&mut self) {
        self.transposition_table.clear();
    }
}

impl Player for AlphaBetaPlayer {
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

    fn get_move(&mut self, board: &core::Board, possible_moves: &Vec<(u8, Vec<u8>)>) -> (u8, u8) {
        let mut best_move = (0, 0);
        let mut best_value = f32::NEG_INFINITY;
        let depth = match self.depth {
            Depth::Static(depth) => depth,
            Depth::Dynamic => {
                let pc = board.black_count() + board.white_count();
                // TODO: Use more sophisticated formula
                if pc < 8 {
                    11
                } else if pc < 15 {
                    8
                } else if pc < 20 {
                    7
                } else {
                    6
                }
            }
        };
        // println!("Depth: {}", depth);
        self.clear_transposition_table();
        for (from, tos) in possible_moves {
            for to in tos {
                let mut new_board = board.clone();
                new_board.move_piece(*from, *to);

                let value =
                    self.alpha_beta(&new_board, depth, false, f32::NEG_INFINITY, f32::INFINITY);
                // println!("{} -> {}: {}", from, to, value);
                if value > best_value {
                    best_value = value;
                    best_move = (*from, *to);
                }
            }
        }

        // println!("{}", self.transposition_table.len());
        // println!("Best move: {} -> {}", best_move.0, best_move.1);
        return best_move;
    }
}
