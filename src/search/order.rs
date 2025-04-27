use crate::{board::Board, evaluation::evaluate_position, moves::{Move, Position}, piece::PieceColor};

use super::{values::*, Search};

impl Search {
    pub(crate) fn sort_moves(&mut self, moves: &Vec<Move>, board: &mut Board) -> Vec<Move> {
        let scores = moves.iter()
            .map(|m| self.evaluate_move(m, board));

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

    pub(crate) fn evaluate_move(&mut self, m: &Move, board: &mut Board) -> f64 {
        let mut value = m.mvv_lva();

        if m.is_promotion {
            value += PROMOTION_VALUE;
        }

        if m.is_castling {
            value += CASTLING_VALUE;
        }

        value += m.ps_table(board);

        value
    }
}

impl Move {
    pub fn mvv_lva(&self) -> f64 {
        if !self.is_capture || self.captured.is_none() {
            return 0.0;
        }

        let captured = self.captured.as_ref().unwrap();

        let victim = captured.piece_type.index();
        let aggressor = self.piece_type.index();

        let value = MVV_LVA_VALUES[victim][aggressor] as f64;

        let victim_value = PIECE_VALUES[victim];
        let aggressor_value = PIECE_VALUES[aggressor];

        if aggressor_value > victim_value {
            let penalty = (aggressor_value - victim_value) * 2.0;
            return value - penalty;
        }

        value
    }

    pub fn ps_table(&self, board: &Board) -> f64 {
        let pos = Position::from_bitboard(self.to);

        let y = if self.color == PieceColor::White { pos.y } else { 7 - pos.y };

        evaluate_position(board, self.piece_type, pos.x, y)
    }
}