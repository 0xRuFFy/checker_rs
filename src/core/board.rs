use std::fmt::{Display, Formatter, Result};

type PieceColor = bool;
const WHITE: PieceColor = true;
const BLACK: PieceColor = false;

type Bitboard = u64;
const DEFAULT_WHITE: Bitboard = 0x000000000055aa55;
const DEFAULT_BLACK: Bitboard = 0xaa55aa0000000000;

type PieceType = bool;
const MAN: PieceType = false;
const KING: PieceType = true;

type Piece = (PieceColor, PieceType);

pub struct Board {
    white: Bitboard,
    black: Bitboard,
    kings: Bitboard,
}

impl Board {
    pub fn new() -> Board {
        return Board {
            white: DEFAULT_WHITE,
            black: DEFAULT_BLACK,
            kings: 0,
        };
    }

    pub fn get_id_from_coords(row: u8, col: u8) -> u8 {
        return row * 8 + col;
    }

    pub fn get_piece(&self, id: u8) -> Option<Piece> {
        let piece_type = self.kings & 1 << id != 0;

        if self.white & 1 << id != 0 {
            return Some((WHITE, piece_type));
        } else if self.black & 1 << id != 0 {
            return Some((BLACK, piece_type));
        }

        return None;
    }

    pub fn get_piece_by_coords(&self, row: u8, col: u8) -> Option<Piece> {
        let id = Board::get_id_from_coords(row, col);

        return self.get_piece(id);
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
