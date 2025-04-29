use crate::{bitboard::{AB_FILE_INV, A_FILE_INV, GH_FILE_INV, H_FILE_INV, RANK_2, RANK_7}, board::Board, piece::{Piece, PieceColor}};

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
        } else {
            if (square & RANK_7) != 0 {
                ((square << 8) << 8) & self.bb.empty & (mask << 8)
            } else {
                0
            }
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
        let knight_moves = ((square << 17) & A_FILE_INV) |
            ((square << 15) & H_FILE_INV) |
            ((square << 10) & AB_FILE_INV) |
            ((square >> 6) & AB_FILE_INV) |
            ((square >> 15) & A_FILE_INV) |
            ((square >> 17) & H_FILE_INV) |
            ((square << 6) & GH_FILE_INV) |
            ((square >> 10) & GH_FILE_INV);

        knight_moves & (self.bb.empty | enemy)
    }

    pub fn get_king_attacks(&self, square: u64, enemy: u64) -> u64 {
        let king_moves = ((square << 1) & A_FILE_INV) |
            ((square >> 1) & H_FILE_INV) |
            (square << 8) |
            (square >> 8) |
            ((square << 9) & A_FILE_INV) |
            ((square << 7) & H_FILE_INV) |
            ((square >> 7) & A_FILE_INV) |
            ((square >> 9) & H_FILE_INV);

        king_moves & (self.bb.empty | enemy)
    }
}