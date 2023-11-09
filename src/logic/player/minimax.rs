use super::Player;
use crate::{
    logic::{
        piece::{self, PieceColor},
        Board, Game, PossibleMoves,
    },
    return_if, return_if_else,
};
use std::collections::HashMap;

const WIN_BASE_VALUE: f32 = 200.;

pub struct MinimaxPlayer {
    color: bool,
    depth: u8,
    eval: fn(&Board, PieceColor) -> f32,
    transposition_table: HashMap<Board, f32>,
    /// The maximum value that can be returned by the evaluation function
    max_value: f32,
}

impl MinimaxPlayer {
    pub fn new(depth: u8, eval: fn(&Board, PieceColor) -> f32) -> Self {
        Self {
            color: piece::WHITE,
            depth,
            eval,
            transposition_table: HashMap::new(),
            max_value: WIN_BASE_VALUE * (depth as f32),
        }
    }

    fn minimax(
        &mut self,
        board: &mut Board,
        depth: u8,
        maximizing_player: bool,
        mut alpha: f32,
        mut beta: f32,
    ) -> f32 {
        if let Some(value) = self.transposition_table.get(board) {
            return *value;
        }
        return_if!(depth == 0, (self.eval)(board, self.color));

        let possible_moves = board.possible_moves(maximizing_player ^ !self.color);
        let d_max_value = WIN_BASE_VALUE * (depth as f32);

        if Game::is_game_over(&possible_moves) {
            // This will favor moves that lead to a win faster
            return_if_else!(maximizing_player, -d_max_value, d_max_value);
        }

        let mut best_value = if maximizing_player {
            -d_max_value
        } else {
            d_max_value
        };

        for p_move in possible_moves {
            for to in p_move.to {
                let move_info = board.move_piece(&p_move.from, &to);
                let value = self.minimax(
                    board,
                    depth - 1,
                    move_info.jumped_piece.is_some() ^ !maximizing_player,
                    alpha,
                    beta,
                );
                board.undo_move(move_info);
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

        // TODO: try not to use clone
        self.transposition_table.insert(board.clone(), best_value);
        best_value
    }
}

impl Player for MinimaxPlayer {
    fn init(&mut self, color: bool) {
        self.color = color;
    }

    fn get_move(&mut self, board: &Board, possible_moves: &[PossibleMoves]) -> (u8, u8) {
        let mut board_clone = board.clone();
        let mut best_move = (0, 0);
        let mut best_value = -self.max_value;

        self.transposition_table.clear();
        for p_move in possible_moves {
            for to in &p_move.to {
                let move_info = board_clone.move_piece(&p_move.from, to);
                let value = self.minimax(
                    &mut board_clone,
                    self.depth,
                    false,
                    -self.max_value,
                    self.max_value,
                );
                println!("({} -> {}) | {}", p_move.from, *to, value);
                board_clone.undo_move(move_info);
                if value > best_value {
                    best_value = value;
                    best_move = (p_move.from, *to);
                }
            }
        }

        println!(
            "Best move: ({} -> {}) | {}",
            best_move.0, best_move.1, best_value
        );
        best_move
    }
}
