use crate::player::Player;
use checkers_core as core;

#[derive(Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Winner(core::board::PieceColor),
    // Draw,
}

pub struct CheckersGame {
    pub board: core::Board,
    pub turn: core::board::PieceColor,
    pub state: GameState,
    pub player: (Box<dyn Player>, Box<dyn Player>),
}

impl CheckersGame {
    pub fn new(mut player1: Box<dyn Player>, mut player2: Box<dyn Player>) -> CheckersGame {
        player1.init(core::board::WHITE);
        player2.init(core::board::BLACK);
        return CheckersGame {
            board: core::Board::new(),
            turn: core::board::WHITE,
            state: GameState::InProgress,
            player: (player1, player2),
        };
    }

    fn check_game_state(&mut self) -> bool {
        if self.board.white_count() == 0 {
            self.state = GameState::Winner(core::board::BLACK);
            return true;
        }
        if self.board.black_count() == 0 {
            self.state = GameState::Winner(core::board::WHITE);
            return true;
        }
        // TODO: Check for draw
        return false;
    }

    fn turn(&mut self) {
        let (from, to) = self.player.0.get_move(&self.board);
        self.board.move_piece(from, to);
        self.turn = !self.turn;
        if self.check_game_state() {
            return;
        }

        let (from, to) = self.player.1.get_move(&self.board);
        self.board.move_piece(from, to);
        self.turn = !self.turn;
        if self.check_game_state() {
            return;
        }
    }

    pub fn play(&mut self) {
        while self.state == GameState::InProgress {
            self.turn();
        }

        match self.state {
            GameState::Winner(color) => println!("{} won!", color),
            // GameState::Draw => println!("Draw!"),
            _ => (),
        }
    }
}
