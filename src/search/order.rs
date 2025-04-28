use crate::{board::Board, evaluation::evaluate_position, moves::{helper::{get_captured, get_color, get_piece_type, get_to, is_capture, is_castling, is_promotion}, Move, Position}, piece::{Piece, PieceColor}};

use super::{values::*, Search};

impl Search {
    pub(crate) fn sort_moves(&mut self, moves: &Vec<Move>, board: &mut Board, depth: u8) -> Vec<(Move, f64)> {
        self.scored_moves.clear();
        self.scored_moves.reserve(moves.len());

        for &m in moves {
            let score = self.evaluate_move(m, board, depth);
            self.scored_moves.push((m, score));
        }

        self.scored_moves.sort_unstable_by(|(_, a), (_, b)| b.total_cmp(a));

        self.scored_moves.clone()
    }

    pub(crate) fn evaluate_move(&mut self, m: Move, board: &mut Board, depth: u8) -> f64 {
        let mut value = mvv_lva(m, board);

        if let Some(entry) = self.tt.get(&board.hash) {
            if let Some(tt_move) = entry.best_move {
                if m == tt_move {
                    value += TT_VALUE;
                }
            }
        }

        if !is_capture(m) {
            if Some(m) == self.killer_moves[depth as usize][0] {
                value += KILLER_MOVE_0;
            } else if Some(m) == self.killer_moves[depth as usize][1] {
                value += KILLER_MOVE_1;
            }

            let piece_index = Piece::index_from(get_piece_type(m), get_color(m));
            let to = get_to(m).trailing_zeros() as usize;

            value += (self.history[piece_index][to] as f64) / HISTORY_VALUE;
        }

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