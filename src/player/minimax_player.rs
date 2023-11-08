use std::collections::HashMap;

use super::{bot_player::Depth, Player};
use crate::{
    game::{CheckersGame, GameState},
    player::bot_player::BotPlayer,
};
use checkers_core as core;

const WIN_BASE_SCORE: f32 = 200.;

pub struct MinimaxPlayer {
    color: core::board::PieceColor,
    depth: Depth,
    eval_version: u8,
    transposition_table: HashMap<core::Board, f32>,
    count: u32,
    max_score: f32,
}

impl MinimaxPlayer {
    pub fn new(depth: &Depth, eval_version: u8) -> Self {
        if ![1, 2].contains(&eval_version) {
            panic!("Invalid eval version");
        }

        MinimaxPlayer {
            color: core::board::WHITE,
            depth: *depth,
            eval_version,
            transposition_table: HashMap::new(),
            count: 0,
            max_score: match depth {
                Depth::Static(d) => WIN_BASE_SCORE * (*d as f32),
            },
        }
    }

    fn minimax(
        &mut self,
        board: &mut core::Board,
        depth: u8,
        maximizing_player: bool,
        mut alpha: f32,
        mut beta: f32,
    ) -> f32 {
        self.count += 1;
        if let Some(value) = self.transposition_table.get(&board) {
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
                    return WIN_BASE_SCORE * (depth as f32);
                } else {
                    return WIN_BASE_SCORE * (depth as f32);
                }
            }
            GameState::InProgress => {}
        }

        let mut best_value = if maximizing_player {
            -WIN_BASE_SCORE * (depth as f32)
        } else {
            WIN_BASE_SCORE * (depth as f32)
        };

        let color = if maximizing_player {
            self.color
        } else {
            !self.color
        };
        let moves = board.get_possible_moves(&color);
        for (from, tos) in moves {
            for to in tos {
                let (jumped, move_) = board.move_piece(from, to);

                let value = self.minimax(
                    board,
                    depth - 1,
                    if jumped {
                        maximizing_player
                    } else {
                        !maximizing_player
                    },
                    alpha,
                    beta,
                );
                board.undo_move(move_);
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

        // TODO: Use zobrist hashing instead of cloning the board
        self.transposition_table.insert(board.clone(), best_value);
        return best_value;
    }

    pub fn clear_transposition_table(&mut self) {
        self.transposition_table.clear();
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

    fn get_move(&mut self, board: &core::Board, possible_moves: &Vec<(u8, Vec<u8>)>) -> (u8, u8) {
        let mut best_move = (0, 0);
        let mut best_value = -self.max_score;
        let depth = match self.depth {
            Depth::Static(depth) => depth,
        };

        self.count = 0;
        self.clear_transposition_table();
        for (from, tos) in possible_moves {
            for to in tos {
                let mut new_board = board.clone();
                new_board.move_piece(*from, *to);

                let value = self.minimax(
                    &mut new_board,
                    depth,
                    false,
                    -self.max_score,
                    self.max_score,
                );
                println!("{} -> {}: {}", from, to, value);
                if value > best_value {
                    best_value = value;
                    best_move = (*from, *to);
                }
            }
        }

        // println!("Best move: {} -> {}", best_move.0, best_move.1);
        println!(
            "Best move: {} -> {}",
            CheckersGame::id_to_chess_notation(best_move.0),
            CheckersGame::id_to_chess_notation(best_move.1)
        );
        return best_move;
    }
}
