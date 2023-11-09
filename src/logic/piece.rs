pub const LEFT_DISTANCE: u8 = 7;
pub const RIGHT_DISTANCE: u8 = 9;

const TOP_LEFT: i8 = LEFT_DISTANCE as i8;
const TOP_RIGHT: i8 = RIGHT_DISTANCE as i8;
const BOTTOM_LEFT: i8 = -TOP_RIGHT;
const BOTTOM_RIGHT: i8 = -TOP_LEFT;

pub type PieceColor = bool;
pub const WHITE: PieceColor = true;
pub const BLACK: PieceColor = false;

pub type PieceType = bool;
pub const KING: PieceType = true;
pub const MAN: PieceType = false;

#[derive(Debug)]
pub struct Piece {
    pub color: PieceColor,
    pub piece_type: PieceType,
}

impl Piece {
    pub fn get_reachable_squares(&self) -> Vec<i8> {
        let dir = self.get_move_direction();
        let mut offsets = vec![TOP_LEFT * dir, TOP_RIGHT * dir];
        if self.piece_type == KING {
            offsets.push(BOTTOM_LEFT * dir);
            offsets.push(BOTTOM_RIGHT * dir);
        }

        offsets
    }

    fn get_move_direction(&self) -> i8 {
        if self.color == WHITE {
            1
        } else {
            -1
        }
    }
}
