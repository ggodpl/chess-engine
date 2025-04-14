use crate::{board::Board, piece::{Piece, PieceColor}};

use super::Move;

impl Board {
    pub fn get_pseudo_legal_moves(&self) {
        let pieces = if self.turn == PieceColor::White {
            self.bb.white_pieces
        } else {
            self.bb.black_pieces
        };

        let mut legal_moves: Vec<Move> = Vec::with_capacity(218);

        let mut rem = pieces;
        while rem != 0 {
            let index = rem.trailing_zeros() as usize;
            let square = 1u64 << index;

            if let Some(piece) = self.bb.get_piece_at(square) {
                todo!()
            }

            rem &= rem - 1;
        }
    }
}