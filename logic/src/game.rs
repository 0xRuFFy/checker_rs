use crate::board::PossibleMoves;
use crate::player::Player;
use crate::{board::Board, break_if, piece};

#[derive(Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Win(piece::PieceColor),
}

pub struct Game {
    pub board: Board,
    pub current: piece::PieceColor,
    pub state: GameState,
    pub player: [Box<dyn Player>; 2],
    current_player: usize,
}

impl Game {
    /* --------------| Constructors |-------------- */
    pub fn new(mut player_1: Box<dyn Player>, mut player_2: Box<dyn Player>) -> Self {
        player_1.init(piece::WHITE);
        player_2.init(piece::BLACK);
        Self {
            board: Board::new(),
            current: piece::WHITE,
            state: GameState::InProgress,
            player: [player_1, player_2],
            current_player: 0,
        }
    }

    /* --------------| Methods |-------------- */

    pub fn play(&mut self) {
        while self.state == GameState::InProgress {
            let possible_moves = self.board.possible_moves(self.current);
            break_if!(self.check_game_state(&possible_moves, self.current));
            self.turn(&possible_moves);
            self.current = !self.current;
        }

        if let GameState::Win(color) = self.state {
            println!("{} wins!", if color { "White" } else { "Black" })
        }
    }

    /* --------------| Static methods |-------------- */
    pub fn is_game_over(possible_moves: &Vec<PossibleMoves>) -> bool {
        if possible_moves.is_empty() {
            return true;
        }

        false
    }

    /* --------------| Private methods |-------------- */
    fn show(&self, _possible_moves: &[PossibleMoves]) {
        println!("{}", self.board);
        // println!("{}", self.board.to_string(&possible_moves));
        // for (i, (from, moves)) in possible_moves.clone().into_iter().enumerate() {
        //     print!("{}: {} -> {{\n", i, Self::id_to_chess_notation(from));
        //     for (j, to) in moves.into_iter().enumerate() {
        //         print!("    {}: {}, \n", j, Self::id_to_chess_notation(to));
        //     }
        //     print!("}}\n");
        // }
        // println!();
    }

    fn check_game_state(
        &mut self,
        possible_moves: &Vec<PossibleMoves>,
        color: piece::PieceColor,
    ) -> bool {
        if possible_moves.is_empty() {
            self.state = GameState::Win(!color);
            return true;
        }

        false
    }

    fn turn(&mut self, possible_moves: &[PossibleMoves]) {
        self.show(possible_moves);
        let (from, to) = self.player[self.current_player].get_move(&self.board, possible_moves);
        let move_info = self.board.move_piece(&from, &to);
        // continue to jump if possible
        if move_info.jumped_piece.is_some() {
            if let Some(possible_jumps) = self.board.possible_jumps_from(to) {
                if !possible_jumps.is_empty() {
                    let possible_moves = PossibleMoves {
                        from: to,
                        to: possible_jumps,
                    };
                    let possible_moves = vec![possible_moves];
                    if !self.check_game_state(&possible_moves, self.current) {
                        self.turn(&possible_moves);
                    }
                }
            }
        }

        self.current_player ^= 1;
    }
}
