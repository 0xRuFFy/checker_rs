use std::fmt::{Display, Formatter, Result};

pub type PieceColor = bool;
pub const WHITE: PieceColor = true;
pub const BLACK: PieceColor = false;

pub type Bitboard = u64;
// const DEFAULT_WHITE: Bitboard = 0x000000000055aa55;
// const DEFAULT_BLACK: Bitboard = 0xaa55aa0000000000;
pub const DEFAULT_WHITE: Bitboard = 0x0000000000550000;
pub const DEFAULT_BLACK: Bitboard = 0x000000000000aa00;

pub type PieceType = bool;
pub const MAN: PieceType = false;
pub const KING: PieceType = true;

type Piece = (PieceColor, PieceType);

pub struct Board {
    white: Bitboard,
    black: Bitboard,

    /// Bitboard of all kings. This is a subset of the white and black bitboards.
    kings: Bitboard,
}

// Note: Most of the following methods [pub(crate)] are not part of the public API. --> the validity of the arguments is not checked. !!!
impl Board {
    // ------------------- CONSTRUCTOR -------------------
    pub fn new() -> Board {
        return Board {
            white: DEFAULT_WHITE,
            black: DEFAULT_BLACK,
            kings: 0x0000000000550000,
            // kings: 0,
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
    fn is_valid_and_empty(&self, id: u8) -> bool {
        return Self::is_valid_id(id) && (self.white | self.black) & 1 << id == 0;
    }

    fn is_valid_and_empty_signed(&self, id: i8) -> bool {
        return Self::is_valid_id_signed(id) && (self.white | self.black) & 1 << id as u8 == 0;
    }

    fn is_valid_and_occupied(&self, id: u8, bitboard: Bitboard) -> bool {
        return Self::is_valid_id(id) && bitboard & 1 << id != 0;
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

    fn get_color_possible_moves(&self, color: PieceColor) -> Vec<(u8, Vec<u8>)> {
        let (count, bitboard) = if color == WHITE {
            (self.white_count() as usize, self.white)
        } else {
            (self.black_count() as usize, self.black)
        };
        if count == 0 {
            return Vec::new();
        }

        let mut moves = Vec::new();

        for i in 0..64 {
            if bitboard & 1 << i != 0 {
                moves.push((i, self.get_possible_moves_of(i).unwrap()));
            }
            if moves.len() == count {
                break;
            }
        }

        return moves;
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

    pub(crate) fn get_piece_by_coords(&self, row: u8, col: u8) -> Option<Piece> {
        let id = Board::get_id_from_coords(row, col);

        return self.get_piece(id);
    }

    pub(crate) fn move_piece(&mut self, from: u8, to: u8) {
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
    }

    pub(crate) fn remove_piece(&mut self, id: u8) {
        self.white &= !(1 << id);
        self.black &= !(1 << id);
        self.kings &= !(1 << id);
    }

    pub(crate) fn king_piece(&mut self, id: u8) {
        self.kings |= 1 << id;
    }

    // TODO
    // pub(crate) fn get_possible_moves_of(&self, id: u8) -> Option<Vec<u8>> {
    pub fn get_possible_moves_of(&self, id: u8) -> Option<Vec<u8>> {
        let piece = self.get_piece(id)?;
        let (color, piece_type) = piece;
        // let dir = if color == WHITE { 1 } else { -1 };

        // let mut moves = Vec::new();

        // let mut jumps = self.get_possible_jumps_for(id, piece_type, color);


        // return Some(self.get_open_squares_for(id, piece_type, color));
        return Some(self.get_possible_jumps_for(id, piece_type, color));
    }

    pub(crate) fn get_possible_moves_of_by_coords(&self, row: u8, col: u8) -> Option<Vec<u8>> {
        return self.get_possible_moves_of(Self::get_id_from_coords(row, col));
    }

    // ------------------- PUBLIC -------------------
    pub fn white_count(&self) -> u8 {
        return self.white.count_ones() as u8;
    }

    pub fn black_count(&self) -> u8 {
        return self.black.count_ones() as u8;
    }

    pub fn white_king_count(&self) -> u8 {
        return self.count_kings(WHITE);
    }

    pub fn black_king_count(&self) -> u8 {
        return self.count_kings(BLACK);
    }

    // TODO: Could use a hashmap to store the possible moves of each piece. (more memory, less computation is expected but needs to be tested in comparison to Vector lookup)
    pub fn get_white_possible_moves(&self) -> Vec<(u8, Vec<u8>)> {
        return self.get_color_possible_moves(WHITE);
    }

    pub fn get_black_possible_moves(&self) -> Vec<(u8, Vec<u8>)> {
        return self.get_color_possible_moves(BLACK);
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
