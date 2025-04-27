use crate::{board::Board, evaluation::evaluate_position, moves::{helper::{get_captured, get_color, get_piece_type, get_to, is_capture, is_castling, is_promotion}, Move, Position}, piece::PieceColor};

use super::{values::*, Search};

impl Search {
    pub(crate) fn sort_moves(&mut self, moves: &Vec<Move>, board: &mut Board) -> Vec<Move> {
        let scores = moves.iter()
            .map(|m| self.evaluate_move(*m, board));

        let mut indices: Vec<(usize, f64)> = scores
            .enumerate()
            .map(|(i, score)| (i, score))
            .collect();

        indices.sort_by(|(_, a), (_, b)| b.total_cmp(a));

        let mut result: Vec<Move> = Vec::with_capacity(moves.len());

        for (i, _) in indices {
            result.push(moves[i].clone());
        }

        result
    }

    pub(crate) fn evaluate_move(&mut self, m: Move, board: &mut Board) -> f64 {
        let mut value = mvv_lva(m, board);

        if is_promotion(m) {
            value += PROMOTION_VALUE;
        }

        if is_castling(m) {
            value += CASTLING_VALUE;
        }

        value += ps_table(m, board);

        value
    }
}

pub fn mvv_lva(m: Move, board: &Board) -> f64 {
    let captured = get_captured(m, board);

    if !is_capture(m) || captured.is_none() {
        return 0.0;
    }

    let captured = captured.as_ref().unwrap();

    let victim = captured.piece_type.index();
    let aggressor = get_piece_type(m).index();

    let value = MVV_LVA_VALUES[victim][aggressor] as f64;

    let victim_value = PIECE_VALUES[victim];
    let aggressor_value = PIECE_VALUES[aggressor];

    if aggressor_value > victim_value {
        let penalty = (aggressor_value - victim_value) * 2.0;
        return value - penalty;
    }

    value
}

pub fn ps_table(m: Move, board: &Board) -> f64 {
    let pos = Position::from_bitboard(get_to(m));

    let y = if get_color(m) == PieceColor::White { pos.y } else { 7 - pos.y };

    evaluate_position(board, get_piece_type(m), pos.x, y)
}