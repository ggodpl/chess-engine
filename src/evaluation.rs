use crate::{bitboard::{A_FILE_INV, H_FILE_INV}, board::Board, piece::{PieceColor, PieceType}, search::values::*};

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

pub fn evaluate(board: &mut Board) -> EvaluationResult {
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
        white: board.bb.count_material(PieceColor::White) as f64 * 2.0,
        black: board.bb.count_material(PieceColor::Black) as f64 * 2.0
    };

    let mobility = evaluate_mobility(board);

    let king_safety = EvaluationResult {
        white: evaluate_king_safety(board, PieceColor::White) * KING_SAFETY_FACTOR,
        black: evaluate_king_safety(board, PieceColor::Black) * KING_SAFETY_FACTOR
    };

    material.combine(mobility)
            .combine(king_safety)
}

pub fn evaluate_mobility(board: &Board) -> EvaluationResult {
    let white_moves = board.bb.white_attacks.count_ones() as f64;
    let black_moves = board.bb.black_attacks.count_ones() as f64;

    EvaluationResult {
        white: white_moves * MOBILITY_VALUE,
        black: black_moves * MOBILITY_VALUE
    }
}

pub fn evaluate_king_safety(board: &Board, color: PieceColor) -> f64 {
    let king = if color == PieceColor::White {
        board.bb.white_king
    } else {
        board.bb.black_king
    };

    let mask = board.attacks.king_attacks[king.trailing_zeros() as usize];

    let shield = mask & if color == PieceColor::White {
        board.bb.white_pawns
    } else {
        board.bb.black_pawns
    };

    let shield_value = shield.count_ones() as f64;

    let breathing_penalty = if (mask & board.bb.pieces).count_ones() >= 3 {
        BREATHING_PENALTY
    } else {
        0.0
    };

    let enemy_pawns = if color == PieceColor::White {
        board.bb.black_pawns
    } else {
        board.bb.white_pawns
    };

    let enemy = if color == PieceColor::White {
        board.bb.black_pieces & !board.bb.black_pawns
    } else {
        board.bb.white_pieces & !board.bb.white_pawns
    };

    let zones = if color == PieceColor::White {
        let zone1 = (king <<  8) | ((king <<  9) & A_FILE_INV) | ((king <<  7) & H_FILE_INV);
        let zone2 = (king << 16) | ((king << 17) & A_FILE_INV) | ((king << 15) & H_FILE_INV);
        let zone3 = (king << 24) | ((king << 25) & A_FILE_INV) | ((king << 23) & H_FILE_INV);

        (zone1, zone2, zone3)
    } else {
        let zone1 = (king >>  8) | ((king >>  9) & A_FILE_INV) | ((king >>  7) & H_FILE_INV);
        let zone2 = (king >> 16) | ((king >> 17) & A_FILE_INV) | ((king >> 15) & H_FILE_INV);
        let zone3 = (king >> 24) | ((king >> 25) & A_FILE_INV) | ((king >> 23) & H_FILE_INV);

        (zone1, zone2, zone3)
    };

    let storm = (zones.0 & enemy_pawns, zones.1 & enemy_pawns, zones.2 & enemy_pawns);
    let storm_value = (storm.0.count_ones() * 3 + storm.1.count_ones() * 2 + storm.2.count_ones() * 1) as f64;
    let storm_penalty = storm_value * PAWN_STORM_PENALTY;

    let storm = (zones.0 & enemy, zones.1 & enemy, zones.2 & enemy);
    let proximity_value = (storm.0.count_ones() * 3 + storm.1.count_ones() * 2 + storm.2.count_ones() * 1) as f64;
    let proximity_penalty = proximity_value * ENEMY_PROXIMITY_PENALTY;

    let virtual_mobility = board.magic.get_queen_moves(king.trailing_zeros() as usize, board.bb.pieces & !king).count_ones();

    let attacked_neighbors = mask & if color == PieceColor::White {
        board.bb.black_attacks
    } else {
        board.bb.white_attacks
    };
    let attack_penalty = attacked_neighbors as f64 * ATTACK_PENALTY;

    let material = if color == PieceColor::White {
        board.bb.count_material(PieceColor::Black) - board.bb.black_pieces.count_ones()
    } else {
        board.bb.count_material(PieceColor::White) - board.bb.white_pieces.count_ones()
    };

    let attack_potential = material as f64 * 0.5;

    const MAX_ATTACK_POTENTIAL: f64 = 13.0;

    let scale_factor = attack_potential / MAX_ATTACK_POTENTIAL;

    let scale = scale_factor.min(0.2);

    let phase = board.calculate_phase();

    let index = king.trailing_zeros() as usize;
    let square = if color == PieceColor::White {
        index
    } else {
        63 - index
    };
    let position_value = (KING_MIDDLEGAME_TABLE[square] * (1.0 - phase)) + (KING_ENDGAME_TABLE[square] * phase);

    let score = shield_value * PAWN_SHIELD_VALUE 
                     + position_value
                     - breathing_penalty * (1.0 - phase)
                     - storm_penalty 
                     - proximity_penalty
                     - virtual_mobility as f64 * VIRTUAL_MOBILITY_PENALTY
                     - attack_penalty;

    if score >= 0.0 {
        score
    } else {
        score * scale
    }
}

pub fn evaluate_positions(board: &Board) -> EvaluationResult {
    let mut white = 0.0;
    let mut black = 0.0;

    let phase = board.calculate_phase();

    let mut rem = board.bb.pieces;
    while rem != 0 {
        let index = rem.trailing_zeros() as usize;
        let piece = board.bb.get_piece_at(1u64 << index).unwrap();

        let square = if piece.color == PieceColor::White {
            index
        } else {
            63 - index
        };

        let value = match piece.piece_type {
            PieceType::Pawn => PAWN_TABLE[square],
            PieceType::Knight => KNIGHT_TABLE[square],
            PieceType::Bishop => BISHOP_TABLE[square],
            PieceType::Rook => ROOK_TABLE[square],
            PieceType::Queen => QUEEN_TABLE[square],
            PieceType::King => (KING_MIDDLEGAME_TABLE[square] * (1.0 - phase)) + (KING_ENDGAME_TABLE[square] * phase)
        };

        if piece.color == PieceColor::White {
            white += value;
        } else {
            black += value;
        }

        rem &= rem - 1;
    }

    EvaluationResult {
        white: white * 0.1,
        black: black * 0.1,
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