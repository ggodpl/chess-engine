use crate::{bitboard::{A_FILE_INV, H_FILE_INV, RANK_2, RANK_7}, board::Board, piece::{Piece, PieceColor}};

impl Board {
    pub fn get_pawn_attacks(&self, square: u64, enemy: u64, piece: Piece) -> u64 {
        let mut mask = if piece.color == PieceColor::White {
            (square >> 8) & self.bb.empty
        } else {
            (square << 8) & self.bb.empty
        };

        mask |= if piece.color == PieceColor::White {
            if (square & RANK_2) != 0 {
                ((square >> 8) >> 8) & self.bb.empty & (mask >> 8)
            } else {
                0
            }
        } else if (square & RANK_7) != 0 {
            ((square << 8) << 8) & self.bb.empty & (mask << 8)
        } else {
            0
        };

        let captures_mask = if piece.color == PieceColor::White {
            ((square >> 9) & H_FILE_INV) | ((square >> 7) & A_FILE_INV)
        } else {
            ((square << 9) & A_FILE_INV) | ((square << 7) & H_FILE_INV)
        };

        mask |= captures_mask & enemy;

        mask | if self.target_square != 0 {
            captures_mask & self.target_square
        } else {
            0
        }
    }

    pub fn get_knight_attacks(&self, square: u64, enemy: u64) -> u64 {
        let knight_moves = self.attacks.knight_attacks[square.trailing_zeros() as usize];

        knight_moves & (self.bb.empty | enemy)
    }

    pub fn get_king_attacks(&self, square: u64, enemy: u64) -> u64 {
        let king_moves = self.attacks.king_attacks[square.trailing_zeros() as usize];

        king_moves & (self.bb.empty | enemy)
    }
}