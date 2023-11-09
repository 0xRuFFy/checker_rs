use crate::logic::{
    piece::{self, PieceColor},
    Board,
};

pub fn v1(board: &Board, color: PieceColor) -> f32 {
    let mut value = 0.;
    let white_multiplier = if color == piece::WHITE { 1. } else { -1. };
    let black_multiplier = if color == piece::BLACK { 1. } else { -1. };
    let king_multiplier = 2.;

    value += board.white_count() as f32 * white_multiplier;
    value += board.white_king_count() as f32 * white_multiplier * king_multiplier;
    value += board.black_count() as f32 * black_multiplier;
    value += board.black_king_count() as f32 * black_multiplier * king_multiplier;

    value
}

pub fn v2(board: &Board, color: PieceColor) -> f32 {
    let mut value = 0.;
    let mut all = board.get_white_black();
    let mut p_count = all.count_ones();

    while p_count > 0 {
        let id = all.trailing_zeros() as u8;
        let piece = board.get_piece(1 << id).unwrap();
        let color_multiplier = if piece.color == color { 1. } else { -1. };
        let position_multiplier: f32;
        if piece.piece_type == piece::KING {
            position_multiplier =
                3. - (((id / 8) as f32 - 3.5).abs() + ((id % 8) as f32 - 3.5).abs()) / 8.;
        } else {
            position_multiplier = match piece.color {
                piece::WHITE => (1 + id / 8) as f32 / 8.,
                piece::BLACK => (8 - id / 8) as f32 / 8.,
            };
        }
        value += color_multiplier * position_multiplier;

        p_count -= 1;
        all &= !(1 << id);
    }

    value
}
