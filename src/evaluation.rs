use crate::{board::Board, piece::{PieceColor, PieceType}, search::values::*};

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

    pub fn combine(&self, other: Self) -> Self {
        EvaluationResult {
            white: self.white + other.white,
            black: self.black + other.black
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

    let material = EvaluationResult {
        white: board.bb.count_material(PieceColor::White) as f64,
        black: board.bb.count_material(PieceColor::Black) as f64
    };

    material
}

pub fn evaluate_mobility(board: &Board) -> EvaluationResult {
    let white_moves = board.get_pseudo_legal_moves(PieceColor::White).len() as f64;
    let black_moves = board.get_pseudo_legal_moves(PieceColor::Black).len() as f64;

    EvaluationResult {
        white: white_moves * MOBILITY_VALUE,
        black: black_moves * MOBILITY_VALUE
    }
}

pub fn evaluate_position(board: &Board, piece_type: PieceType, x: usize, y: usize) -> f64 {
    match piece_type {
        PieceType::Pawn => PAWN_TABLE[y * 8 + x],
        PieceType::Knight => KNIGHT_TABLE[y * 8 + x],
        PieceType::Bishop => BISHOP_TABLE[y * 8 + x],
        PieceType::Rook => ROOK_TABLE[y * 8 + x],
        PieceType::Queen => QUEEN_TABLE[y * 8 + x],
        PieceType::King => {
            let phase = board.calculate_phase();
            (KING_MIDDLEGAME_TABLE[y * 8 + x] * (1.0 - phase)) + (KING_ENDGAME_TABLE[y * 8 + x] * phase)
        }
    }
}

impl Board {
    pub fn calculate_phase(&self) -> f64 {
        let mut phase = MAX_PHASE;

        phase -= (self.bb.white_knights | self.bb.black_knights | self.bb.white_bishops | self.bb.black_bishops).count_ones() as i32
            + (self.bb.white_rooks | self.bb.black_rooks).count_ones() as i32 * 2
            + (self.bb.white_queens | self.bb.black_queens).count_ones() as i32 * 4;

        phase = phase.clamp(0, MAX_PHASE);

        phase as f64 / MAX_PHASE as f64
    }
}