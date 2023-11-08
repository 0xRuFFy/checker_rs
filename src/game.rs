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
    pub player: [Box<dyn Player>; 2],
    current_player: usize,
}

impl CheckersGame {
    pub fn new(mut player1: Box<dyn Player>, mut player2: Box<dyn Player>) -> CheckersGame {
        player1.init(core::board::WHITE);
        player2.init(core::board::BLACK);
        return CheckersGame {
            board: core::Board::new(),
            turn: core::board::WHITE,
            state: GameState::InProgress,
            player: [player1, player2],
            current_player: 0,
        };
    }

    pub fn id_to_chess_notation(id: u8) -> String {
        let row = (id / 8) + 1;
        let col = (id % 8) + 1;
        let col = match col {
            1 => "a",
            2 => "b",
            3 => "c",
            4 => "d",
            5 => "e",
            6 => "f",
            7 => "g",
            8 => "h",
            _ => panic!("Invalid column"),
        };
        return format!("{}{}", col, row);
    }

    fn show(&self, possible_moves: &Vec<(u8, Vec<u8>)>) {
        println!("{}", self.board.to_string(&possible_moves));
        for (i, (from, moves)) in possible_moves.clone().into_iter().enumerate() {
            print!("{}: {} -> {{\n", i, Self::id_to_chess_notation(from));
            for (j, to) in moves.into_iter().enumerate() {
                print!("    {}: {}, \n", j, Self::id_to_chess_notation(to));
            }
            print!("}}\n");
        }
        println!();
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

    // TODO: Remove code duplication between player.0 and player.1
    fn turn(&mut self) {
        let possible_moves = self.board.get_possible_moves(&self.turn);
        if possible_moves.len() == 0 {
            self.state = GameState::Winner(!self.turn);
            return;
        }
        self.show(&possible_moves);
        let (from, to) = self.player[self.current_player].get_move(&self.board, &possible_moves);
        let (mut jumped, _) = self.board.move_piece(from, to);
        while jumped {
            if self.check_game_state() {
                return;
            }
            if let Some(possible_jumps) = self.board.get_possible_jumps_of(to) {
                if possible_jumps.len() == 0 {
                    break;
                }
                let possible_jumps = vec![(to, possible_jumps)];
                self.show(&possible_jumps);
                let (from, to) =
                    self.player[self.current_player].get_move(&self.board, &possible_jumps);
                jumped = self.board.move_piece(from, to).0;
            } else {
                break;
            }
        }

        self.turn = !self.turn;
        self.current_player = (self.current_player + 1) % 2;
        self.check_game_state();
    }

    pub fn play(&mut self) {
        while self.state == GameState::InProgress {
            self.turn();
        }

        match self.state {
            GameState::Winner(color) => println!("{} won!", if color { "White" } else { "Black" }),
            // GameState::Draw => println!("Draw!"),
            _ => (),
        }
    }
}
