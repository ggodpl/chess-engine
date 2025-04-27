use crate::{board::Board, piece::PieceColor};

pub struct EvaluationResult {
    pub white: f64,
    pub black: f64
}

impl EvaluationResult {
    pub fn to_value(&self) -> f64 {
        self.white - self.black
    }

    pub fn default() -> Self {
        EvaluationResult {
            white: 0.0,
            black: 0.0
        }
    }
}

pub fn evaluate(board: &Board) -> EvaluationResult {
    if board.is_checkmate() {
        return if board.turn == PieceColor::White {
            EvaluationResult {
                white: 100000000.0,
                black: 0.0
            }
        } else {
            EvaluationResult {
                white: 0.0,
                black: 100000000.0
            }
        }
    }

    if board.is_draw() {
        return EvaluationResult::default();
    }

    EvaluationResult {
        white: board.bb.count_material(PieceColor::White) as f64,
        black: board.bb.count_material(PieceColor::Black) as f64
    }
}