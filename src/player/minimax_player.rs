use std::collections::HashMap;

use super::{bot_player::Depth, Player};
use crate::{game::GameState, player::bot_player::BotPlayer};
use checkers_core as core;

pub struct MinimaxPlayer {
    color: core::board::PieceColor,
    depth: Depth,
    eval_version: u8,
    transposition_table: HashMap<core::Board, f32>,
}

impl MinimaxPlayer {
    pub fn new(depth: Depth, eval_version: u8) -> Self {
        if ![1, 2].contains(&eval_version) {
            panic!("Invalid eval version");
        }
        MinimaxPlayer {
            color: core::board::WHITE,
            depth,
            eval_version,
            transposition_table: HashMap::new(),
        }
    }

    fn minimax(&mut self, board: &core::Board, depth: u8, maximizing_player: bool) -> f32 {
        if let Some(value) = self.transposition_table.get(board) {
            // println!("Cache hit");
            // TODO: Cache is not updated -> cache position must not have been calculated with the same depth
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
                new_board.move_piece(from, to);

                let value = self.minimax(&new_board, depth - 1, !maximizing_player);
                self.transposition_table.insert(new_board, value);
                if maximizing_player {
                    best_value = best_value.max(value);
                } else {
                    best_value = best_value.min(value);
                }
            }
        }

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
        let mut best_value = f32::NEG_INFINITY;
        let depth = match self.depth {
            Depth::Static(depth) => depth,
            Depth::Dynamic => {
                let pmc = possible_moves
                    .iter()
                    .map(|(_, tos)| tos.len())
                    .sum::<usize>();
                // TODO: Use more sophisticated formula
                if pmc < 2 {
                    12
                } else if pmc < 4 {
                    10
                } else {
                    7
                }
            }
        };
        for (from, tos) in possible_moves {
            for to in tos {
                let mut new_board = board.clone();
                new_board.move_piece(*from, *to);

                let value = self.minimax(&new_board, depth, false);
                println!("{} -> {}: {}", from, to, value);
                if value > best_value {
                    best_value = value;
                    best_move = (*from, *to);
                }
            }
        }

        // println!("{}", self.transposition_table.len());
        return best_move;
    }
}
