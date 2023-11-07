use std::fmt::{Display, Formatter, Result};
use colored::*;

pub type PieceColor = bool;
pub const WHITE: PieceColor = true;
pub const BLACK: PieceColor = false;

pub type Bitboard = u64;
const DEFAULT_WHITE: Bitboard = 0x000000000055aa55;
const DEFAULT_BLACK: Bitboard = 0xaa55aa0000000000;

pub type PieceType = bool;
pub const MAN: PieceType = false;
pub const KING: PieceType = true;

type Piece = (PieceColor, PieceType);

pub type MoveType = bool;
pub const MOVE: MoveType = false;
pub const JUMP: MoveType = true;

#[derive(Clone)]
pub struct Board {
    white: Bitboard,
    black: Bitboard,

    /// Bitboard of all kings. This is a subset of the white and black bitboards.
    kings: Bitboard,
}

// Note: Most of the following methods are not checked for validity of their arguments. !!!
impl Board {
    // ------------------- CONSTRUCTOR -------------------
    pub fn new() -> Board {
        return Board {
            white: DEFAULT_WHITE,
            black: DEFAULT_BLACK,
            kings: 0,
        };
    }

    // ------------------- STATIC -------------------
    pub(crate) fn get_id_from_coords(row: u8, col: u8) -> u8 {
        return row * 8 + col;
    }

    pub(crate) fn is_valid_id(id: u8) -> bool {
        return id < 64 && (id % 8 + id / 8) % 2 == 0;
    }

    pub(crate) fn is_valid_id_signed(id: i8) -> bool {
        return id >= 0 && Self::is_valid_id(id as u8);
    }

    // ------------------- PRIVATE -------------------
    fn is_valid_and_empty_signed(&self, id: i8) -> bool {
        return Self::is_valid_id_signed(id) && (self.white | self.black) & 1 << id as u8 == 0;
    }

    fn is_valid_and_occupied_signed(&self, id: i8, bitboard: Bitboard) -> bool {
        return Self::is_valid_id_signed(id) && bitboard & 1 << id as u8 != 0;
    }

    fn count_kings(&self, color: PieceColor) -> u8 {
        return (if color == WHITE {
            self.white
        } else {
            self.black
        } & self.kings)
            .count_ones() as u8;
    }

    // Note: this is compact but may be slow.
    fn get_open_squares_for(&self, id: u8, piece_type: PieceType, color: PieceColor) -> Vec<u8> {
        let dir = if color == WHITE { 1 } else { -1 };

        let mut s = vec![7, 9];
        if piece_type == KING {
            s.append(&mut vec![-7, -9]);
        }
        return s
            .into_iter()
            .map(|x| id as i8 + x * dir)
            .filter(|x| self.is_valid_and_empty_signed(*x))
            .map(|x| x as u8)
            .collect();
    }

    fn get_possible_jumps_for(&self, id: u8, piece_type: PieceType, color: PieceColor) -> Vec<u8> {
        let dir = if color == WHITE { 1 } else { -1 };
        let occ = if color == WHITE {
            self.black
        } else {
            self.white
        };

        let mut s = vec![7, 9];
        if piece_type == KING {
            s.append(&mut vec![-7, -9]);
        }
        return s
            .into_iter()
            .map(|x| (id as i8 + x * dir, x * dir))
            .filter(|x| self.is_valid_and_occupied_signed((*x).0, occ) && self.is_valid_and_empty_signed((*x).0 + (*x).1))
            .map(|x| (x.0 + x.1) as u8)
            .collect();
    }

    // ------------------- PUB(CRATE) -------------------
    pub(crate) fn get_piece(&self, id: u8) -> Option<Piece> {
        let piece_type = self.kings & 1 << id != 0;

        if self.white & 1 << id != 0 {
            return Some((WHITE, piece_type));
        } else if self.black & 1 << id != 0 {
            return Some((BLACK, piece_type));
        }

        return None;
    }

    pub(crate) fn get_possible_moves_of(&self, id: u8) -> Option<(Vec<u8>, MoveType)> {
        let piece = self.get_piece(id)?;
        let (color, piece_type) = piece;

        let jumps = self.get_possible_jumps_for(id, piece_type, color);
        if jumps.len() > 0 {
            return Some((jumps, JUMP));
        }

        return Some((self.get_open_squares_for(id, piece_type, color), MOVE));
    }

    // ------------------- PUBLIC -------------------
    pub fn to_string(&self, possible_moves: &Vec<(u8, Vec<u8>)>) -> String {
        let mut board = String::new();

        for row in (0..8).rev() {
            for col in 0..8 {
                let highlight = possible_moves.iter().any(|x| x.0 == Self::get_id_from_coords(row, col));
                board.push_str(
                    format!(
                        "|{}",
                        match self.get_piece_by_coords(row, col) {
                            Some((WHITE, MAN)) => 'w',
                            Some((BLACK, MAN)) => 'b',
                            Some((WHITE, KING)) => 'W',
                            Some((BLACK, KING)) => 'B',
                            None => ' ',
                        }.to_string().color(if highlight { Color::Green } else { Color::White })
                    )
                    .as_str(),
                );
            }
            board.push_str("|\n");
        }

        return board;
    }
    
    pub fn get_piece_by_coords(&self, row: u8, col: u8) -> Option<Piece> {
        let id = Board::get_id_from_coords(row, col);

        return self.get_piece(id);
    }

    #[inline]
    pub fn white_count(&self) -> u8 {
        return self.white.count_ones() as u8;
    }

    #[inline]
    pub fn black_count(&self) -> u8 {
        return self.black.count_ones() as u8;
    }

    #[inline]
    pub fn white_king_count(&self) -> u8 {
        return self.count_kings(WHITE);
    }

    #[inline]
    pub fn black_king_count(&self) -> u8 {
        return self.count_kings(BLACK);
    }

    // TODO: Could use a hashmap to store the possible moves of each piece. (more memory, less computation is expected but needs to be tested in comparison to Vector lookup)
    pub fn get_possible_moves(&self, color: &PieceColor) -> Vec<(u8, Vec<u8>)> {
        let (mut count, bitboard) = if *color == WHITE {
            (self.white_count() as usize, self.white)
        } else {
            (self.black_count() as usize, self.black)
        };
        if count == 0 {
            return Vec::new();
        }

        let mut moves = Vec::new();
        let mut jumps = Vec::new();
        let mut can_jump = false;

        for i in 0..64 {
            if bitboard & 1 << i != 0 {
                if let Some((moves_of_i, move_type)) = self.get_possible_moves_of(i) {
                    count -= 1;
                    can_jump |= move_type;
                    if moves_of_i.len() > 0 {
                        if move_type == JUMP {
                            jumps.push((i, moves_of_i))
                        } else {
                            moves.push((i, moves_of_i))
                        }
                    }
                }
                if count == 0 {
                    break;
                }
            }
        }

        return if can_jump { jumps } else { moves };
    }

    pub fn get_possible_jumps_of(&self, id: u8) -> Option<Vec<u8>> {
        let piece = self.get_piece(id)?;
        let (color, piece_type) = piece;

        return Some(self.get_possible_jumps_for(id, piece_type, color));
    }

    pub fn move_piece(&mut self, from: u8, to: u8) -> bool {
        if self.white & 1 << from != 0 {
            self.white &= !(1 << from);
            self.white |= 1 << to;
        } else if self.black & 1 << from != 0 {
            self.black &= !(1 << from);
            self.black |= 1 << to;
        }

        if self.kings & 1 << from != 0 {
            self.kings &= !(1 << from);
            self.kings |= 1 << to;
        }

        let mut jumped = true;
        // remove jumped pieces
        if (from as i8 - to as i8).abs() == 18 {
            self.remove_piece((from + to) / 2);
        } else if (from as i8 - to as i8).abs() == 14 {
            self.remove_piece((from + to) / 2);
        } else {
            jumped = false;
        }

        // promote to king
        if self.white & 1 << to != 0 && to / 8 == 7 {
            self.king_piece(to);
        } else if self.black & 1 << to != 0 && to / 8 == 0 {
            self.king_piece(to);
        }

        return jumped;
    }

    pub fn remove_piece(&mut self, id: u8) {
        self.white &= !(1 << id);
        self.black &= !(1 << id);
        self.kings &= !(1 << id);
    }

    pub fn king_piece(&mut self, id: u8) {
        self.kings |= 1 << id;
    }

    pub fn eval_v1(&self, color: PieceColor) -> f32 {
        let mut score = 0.;
        let white_multiplier = if color == WHITE { 1 } else { -1 };
        let black_multiplier = if color == WHITE { -1 } else { 1 };

        score += (self.white_count() as i32 * white_multiplier) as f32;
        score += (self.white_king_count() as i32 * white_multiplier * 2) as f32;
        score += (self.black_count() as i32 * black_multiplier) as f32;
        score += (self.black_king_count() as i32 * black_multiplier * 2) as f32;

        return score;
    }

    // Version 2 of the evaluation function will value MANs more if they are closer to their KING row
    // and will value KINGs more if they are closer to the center.
    pub fn eval_v2(&self, color: PieceColor) -> f32 {
        let mut score = 0.;
        for i in 0..8 {
            for j in 0..8 {
                if let Some((piece_color, piece_type)) = self.get_piece_by_coords(i, j) {
                    let color_multiplier = if piece_color == color { 1. } else { -1. };
                    let position_multiplier: f32;
                    if piece_type == KING {
                        position_multiplier = 3. - ((i as f32 - 3.5).abs() + (j as f32 - 3.5).abs()) / 8.;
                    } else {
                        position_multiplier = if piece_color == WHITE {
                            (1 + i) as f32 / 8.
                        } else {
                            (8 - i) as f32 / 8.
                        };
                    }
                    score += color_multiplier * position_multiplier;
                }
            }
        }
        return score;
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut board = String::new();

        for row in (0..8).rev() {
            for col in 0..8 {
                board.push_str(
                    format!(
                        "|{}",
                        match self.get_piece_by_coords(row, col) {
                            Some((WHITE, MAN)) => 'w',
                            Some((BLACK, MAN)) => 'b',
                            Some((WHITE, KING)) => 'W',
                            Some((BLACK, KING)) => 'B',
                            None => ' ',
                        }
                    )
                    .as_str(),
                );
            }
            board.push_str("|\n");
        }

        write!(f, "{}", board)
    }
}
