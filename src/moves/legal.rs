use crate::{board::Board, piece::PieceColor};

use super::Move;

impl Board {
    pub fn filter_legal_moves(&self, moves: &mut Vec<Move>) {
        if self.is_double_checked(self.turn) {
            moves.retain(|m| m.is_king && !self.is_attacked(m.to, self.turn.opposite()));
            return;
        }

        if self.is_checked(self.turn) {
            moves.retain(|m| {
                if m.is_king {
                    !self.is_attacked(m.to, self.turn.opposite())
                } else {
                    let king = if self.turn == PieceColor::White {
                        self.bb.white_king
                    } else {
                        self.bb.black_king
                    };

                    // when a piece is pinned, its not able to block a check, no matter how the position looks
                    !self.is_pinned(m.from) && self.attacks.is_between(m.to, self.get_attackers(king, self.turn.opposite()), king)
                }
            });
            return;
        }

        moves.retain(|m| {
            let king = if self.turn == PieceColor::White {
                self.bb.white_king
            } else {
                self.bb.black_king
            };
            
            if m.is_en_passant {
                // handle phantom pins
                let line = self.attacks.get_line_mask(m.from, king);
                let ray = self.attacks.get_ray(m.from, king);

                let complement = line & !ray & !m.from & !king;
                
                let captured = if self.turn == PieceColor::White {
                    m.to << 8
                } else {
                    m.to >> 8
                };

                let occupancy = self.bb.pieces & !captured;
                
                let attackers = self.magic.get_queen_moves(m.from.trailing_zeros() as usize, occupancy);
                
                let enemy = if self.turn == PieceColor::White {
                    self.bb.black_pieces
                } else {
                    self.bb.white_pieces
                };

                return complement & attackers & enemy == 0
            }

            let pin = self.get_pin(m.from);

            if pin != 0 {
                let line = self.attacks.get_line_mask(m.from, king);

                m.to & line != 0
            } else {
                true
            }
        });
    }

    pub fn get_pin(&self, square: u64) -> u64 {
        let piece = self.bb.get_piece_at(square);
        if piece.is_none() {
            return 0;
        }

        let piece = piece.unwrap();

        let king = if piece.color == PieceColor::White {
            self.bb.white_king
        } else {
            self.bb.black_king
        };

        let line = self.attacks.get_line_mask(square, king);
        let ray = self.attacks.get_ray(square, king);

        if line == 0 { return 0; }

        let complement = line & !ray & !square & !king;
    
        let enemy = if piece.color == PieceColor::White {
            self.bb.black_pieces
        } else {
            self.bb.white_pieces
        };

        let pinners = complement & enemy;

        let mut result = 0;
        let mut rem = pinners;
        while rem != 0 {
            let index = rem.trailing_zeros() as usize;
            let pin = 1u64 << index;

            if self.attacks.get_ray(pin, square) & self.bb.pieces == 0 {
                result |= pin;
            }

            rem &= rem - 1;
        }

        result
    }

    pub fn is_pinned(&self, square: u64) -> bool {
        self.get_pin(square) != 0
    }
}