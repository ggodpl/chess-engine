use crate::{board::Board, piece::{PieceColor, PieceType}};

use super::Move;

impl Board {
    pub fn filter_legal_moves(&self, moves: &mut Vec<Move>) {
        moves.retain(|m| if m.piece_type == PieceType::King { !self.is_attacked(m.to, self.turn.opposite()) } else { true });
        
        if self.is_double_checked(self.turn) {
            moves.retain(|m| m.piece_type == PieceType::King);
            return;
        }

        if self.is_checked(self.turn) {
            moves.retain(|m| {
                if m.piece_type == PieceType::King {
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
                
                let captured = if self.turn == PieceColor::White {
                    m.to << 8
                } else {
                    m.to >> 8
                };

                let occupancy = self.bb.pieces & !captured;

                let complement = line & !ray & !m.from & !king;
                        
                let bishop_attackers = self.magic.get_bishop_moves(m.from.trailing_zeros() as usize, occupancy);
                
                let rook_attackers = self.magic.get_rook_moves(m.from.trailing_zeros() as usize, occupancy);
        
                let enemy_bishops = if self.turn == PieceColor::White {
                    self.bb.black_bishops | self.bb.black_queens
                } else {
                    self.bb.white_bishops | self.bb.white_queens
                };
        
                let enemy_rooks = if self.turn == PieceColor::White {
                    self.bb.black_rooks | self.bb.black_queens
                } else {
                    self.bb.white_rooks | self.bb.white_queens
                };
        
                let attackers = (bishop_attackers & enemy_bishops) | (rook_attackers & enemy_rooks);
        
                return complement & attackers == 0;
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
        // a piece is blocking the pin
        if ray & !square & !king & self.bb.pieces != 0 { return 0; }

        let complement = line & !ray & !square & !king;
                
        let bishop_attackers = self.magic.get_bishop_moves(square.trailing_zeros() as usize, self.bb.pieces);
        
        let rook_attackers = self.magic.get_rook_moves(square.trailing_zeros() as usize, self.bb.pieces);

        let enemy_bishops = if self.turn == PieceColor::White {
            self.bb.black_bishops | self.bb.black_queens
        } else {
            self.bb.white_bishops | self.bb.white_queens
        };

        let enemy_rooks = if self.turn == PieceColor::White {
            self.bb.black_rooks | self.bb.black_queens
        } else {
            self.bb.white_rooks | self.bb.white_queens
        };

        let attackers = (bishop_attackers & enemy_bishops) | (rook_attackers & enemy_rooks);

        complement & attackers
    }

    pub fn is_pinned(&self, square: u64) -> bool {
        self.get_pin(square) != 0
    }
}