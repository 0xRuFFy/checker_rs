use super::piece::{PieceColor, LEFT_DISTANCE, RIGHT_DISTANCE};
use crate::{
    logic::{piece, Piece},
    return_if,
};
use std::{
    fmt::{Display, Formatter, Result},
    hash::{Hash, Hasher},
};

pub type Bitboard = u64;
const EMPTY: Bitboard = 0;
const DEFAULT_WHITE: Bitboard = 0x0000_0000_0055_AA55;
const DEFAULT_BLACK: Bitboard = 0xAA55_AA00_0000_0000;
// const DEFAULT_WHITE: Bitboard = 0x0000_0000_0055_AA55;
// const DEFAULT_BLACK: Bitboard = 0x0000_0000_AA00_0000;

type MoveType = bool;
const JUMP: MoveType = true;
const STANDARD: MoveType = false;

/// All possible moves for a piece.
#[derive(Debug)]
pub struct PossibleMoves {
    pub from: u8,
    pub to: Vec<u8>,
}

/// Information about a move.
/// Used to undo a move.
#[derive(Debug)]
pub struct MoveInfo {
    pub from: u8,
    pub to: u8,
    pub jumped_piece: Option<Piece>,
    pub jumped_piece_id: Option<u8>,
    pub crowned: bool,
}

/// A Checkers board.
#[derive(Clone)]
pub struct Board {
    /// Bitboard of all white pieces.
    white: Bitboard,

    /// Bitboard of all black pieces.
    black: Bitboard,

    /// Bitboard of all kings. This is a subset of `white` and `black`.
    kings: Bitboard,
}

impl Board {
    /* --------------| Constructors |-------------- */
    pub fn new() -> Self {
        Self {
            white: DEFAULT_WHITE,
            black: DEFAULT_BLACK,
            kings: EMPTY,
        }
    }

    /* --------------| Static methods |-------------- */
    pub fn coords_to_bitboard(row: u8, col: u8) -> Bitboard {
        return 1 << (row * 8 + col);
    }

    /* --------------| Getters |-------------- */
    /// Returns a bitboard of all white and black pieces.
    pub fn get_white_black(&self) -> Bitboard {
        self.white | self.black
    }

    /// bitboard is only allowed to have one bit set
    pub fn get_piece(&self, bitboard: Bitboard) -> Option<Piece> {
        if bitboard.count_ones() != 1 {
            return None;
        }

        let p_color = self.white & bitboard != EMPTY;
        if !p_color && self.black & bitboard == EMPTY {
            return None;
        }

        Some(Piece {
            color: p_color,
            piece_type: self.kings & bitboard != EMPTY,
        })
    }

    /* --------------| Setters |-------------- */

    /* --------------| Methods |-------------- */
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
        return self.count_kings(piece::WHITE);
    }

    #[inline]
    pub fn black_king_count(&self) -> u8 {
        return self.count_kings(piece::BLACK);
    }

    pub fn possible_moves(&self, color: PieceColor) -> Vec<PossibleMoves> {
        let (mut count, mut bitboard) = match color {
            piece::WHITE => (self.white.count_ones(), self.white),
            piece::BLACK => (self.black.count_ones(), self.black),
        };
        let mut moves = Vec::new();
        return_if!(count == 0, moves);

        let mut jumps = Vec::new();
        let mut force_jump = false;

        while count > 0 {
            let from = bitboard.trailing_zeros() as u8;
            let (tos, move_type) = self.get_possible_moves_from(from).unwrap();
            force_jump |= move_type == JUMP;
            if tos.len() > 0 {
                if move_type == JUMP {
                    jumps.push(PossibleMoves { from, to: tos });
                } else if !force_jump {
                    moves.push(PossibleMoves { from, to: tos });
                }
            }

            count -= 1;
            bitboard &= !(1 << from);
        }

        if force_jump {
            return jumps;
        }

        moves
    }

    // Note: this method should be used to calculate the consecutive jumps.
    pub fn possible_jumps_from(&self, from: u8) -> Option<Vec<u8>> {
        let piece = self.get_piece(1 << from)?;
        Some(self.get_jumps_from(&from, &piece))
    }

    /// Does a move on the board.
    /// ! This method does not check if the move is valid !
    /// Returns information about the move.
    pub fn move_piece(&mut self, from: &u8, to: &u8) -> MoveInfo {
        let from_bitboard = 1 << from;
        let to_bitboard = 1 << to;

        // Move piece depending on color
        if self.white & from_bitboard != EMPTY {
            self.white &= !from_bitboard;
            self.white |= to_bitboard;
        } else {
            self.black &= !from_bitboard;
            self.black |= to_bitboard;
        }

        // Move king if necessary
        if self.kings & from_bitboard != EMPTY {
            self.kings &= !from_bitboard;
            self.kings |= to_bitboard;
        }

        // Check for jumped piece
        let mut jumped_piece = None;
        let mut jumped_piece_id = None;
        let move_distance = from.abs_diff(*to);
        if move_distance == LEFT_DISTANCE * 2 || move_distance == RIGHT_DISTANCE * 2 {
            if let Some(piece) = self.get_piece(1 << (from + to) / 2) {
                jumped_piece = Some(piece);
                jumped_piece_id = Some((from + to) / 2);
                self.remove_piece(&jumped_piece_id.unwrap());
            }
        }

        // Check for crowned piece
        let mut crowned = false;
        if (self.white & to_bitboard != EMPTY && to / 8 == 7)
            || (self.black & to_bitboard != EMPTY && to / 8 == 0)
        {
            self.kings |= to_bitboard;
            crowned = true;
        }

        MoveInfo {
            from: *from,
            to: *to,
            jumped_piece,
            jumped_piece_id,
            crowned,
        }
    }

    pub fn undo_move(&mut self, move_info: MoveInfo) {
        let from_bitboard = 1 << move_info.from;
        let to_bitboard = 1 << move_info.to;

        // Move piece depending on color
        if self.white & to_bitboard != EMPTY {
            self.white &= !to_bitboard;
            self.white |= from_bitboard;
        } else {
            self.black &= !to_bitboard;
            self.black |= from_bitboard;
        }

        // Move king if necessary
        if self.kings & to_bitboard != EMPTY {
            self.kings &= !to_bitboard;
            self.kings |= from_bitboard;
        }

        if let Some(jumped_piece_id) = move_info.jumped_piece_id {
            self.add_piece(&jumped_piece_id, &move_info.jumped_piece.unwrap());
        }

        if move_info.crowned {
            self.kings &= !from_bitboard;
        }
    }

    /* --------------| Private methods |-------------- */
    fn is_valid_id(id: &i8) -> bool {
        return *id >= 0 && *id < 64 && (*id % 8 + *id / 8) % 2 == 0;
    }

    fn is_valid_and_empty(&self, id: &i8) -> bool {
        return Self::is_valid_id(id) && (self.white | self.black) & 1 << (*id) as u8 == 0;
    }

    fn get_opponent(&self, color: &PieceColor) -> &Bitboard {
        match color {
            &piece::WHITE => &self.black,
            &piece::BLACK => &self.white,
        }
    }

    fn count_kings(&self, color: PieceColor) -> u8 {
        return (if color == piece::WHITE {
            self.white
        } else {
            self.black
        } & self.kings)
            .count_ones() as u8;
    }

    fn get_open_squares_from(&self, from: &u8, piece: &Piece) -> Vec<u8> {
        let squares = piece.get_reachable_squares();

        squares
            .into_iter()
            .map(|id| id + *from as i8)
            .filter(|id| self.is_valid_and_empty(id))
            .map(|id| id as u8)
            .collect()
    }

    fn get_jumps_from(&self, from: &u8, piece: &Piece) -> Vec<u8> {
        let opponent = self.get_opponent(&piece.color);
        let squares = piece.get_reachable_squares();

        squares
            .into_iter()
            .map(|id| (id + *from as i8, id * 2 + *from as i8))
            .filter(|id| {
                Self::is_valid_id(&id.0)
                    && opponent & 1 << id.0 != 0
                    && self.is_valid_and_empty(&id.1)
            })
            .map(|id| id.1 as u8)
            .collect()
    }

    fn get_possible_moves_from(&self, from: u8) -> Option<(Vec<u8>, MoveType)> {
        let piece = self.get_piece(1 << from)?;

        let jumps = self.get_jumps_from(&from, &piece);
        if jumps.len() > 0 {
            return Some((jumps, JUMP));
        }

        return Some((self.get_open_squares_from(&from, &piece), STANDARD));
    }

    fn add_piece(&mut self, id: &u8, piece: &Piece) {
        let bitboard = 1 << id;
        if piece.color == piece::WHITE {
            self.white |= bitboard;
        } else {
            self.black |= bitboard;
        }

        if piece.piece_type == piece::KING {
            self.kings |= bitboard;
        }
    }

    fn remove_piece(&mut self, id: &u8) {
        let bitboard = 1 << id;
        self.white &= !bitboard;
        self.black &= !bitboard;
        self.kings &= !bitboard;
    }

    /* --------------| Evaluators |-------------- */
}

impl Hash for Board {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.white.hash(state);
        self.black.hash(state);
        self.kings.hash(state);
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.white == other.white && self.black == other.black && self.kings == other.kings
    }
}
impl Eq for Board {}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut board = String::new();

        for row in (0..8).rev() {
            for col in 0..8 {
                board.push_str(
                    format!(
                        "|{}",
                        match self.get_piece(Board::coords_to_bitboard(row, col)) {
                            Some(Piece {
                                color: piece::WHITE,
                                piece_type: piece::MAN,
                            }) => 'w',
                            Some(Piece {
                                color: piece::BLACK,
                                piece_type: piece::MAN,
                            }) => 'b',
                            Some(Piece {
                                color: piece::WHITE,
                                piece_type: piece::KING,
                            }) => 'W',
                            Some(Piece {
                                color: piece::BLACK,
                                piece_type: piece::KING,
                            }) => 'B',
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
