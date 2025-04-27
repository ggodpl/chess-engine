use crate::{board::Board, piece::{PieceColor, PieceType}};

use super::{helper::{get_from, get_piece_type, get_to, is_en_passant}, Move};

impl Board {
    pub fn filter_legal_moves(&self, moves: &mut Vec<Move>) {
        moves.retain(|m| if get_piece_type(*m) == PieceType::King { !self.is_attacked(get_to(*m), self.turn.opposite()) } else { true });
        
        if self.is_double_checked(self.turn) {
            moves.retain(|m| get_piece_type(*m) == PieceType::King);
            return;
        }

        if self.is_checked(self.turn) {
            moves.retain(|m| {
                let to = get_to(*m);
                let from = get_from(*m);
                if get_piece_type(*m) == PieceType::King {
                    !self.is_attacked(to, self.turn.opposite())
                } else {
                    let king = if self.turn == PieceColor::White {
                        self.bb.white_king
                    } else {
                        self.bb.black_king
                    };

                    if is_en_passant(*m) {
                        let captured = if self.turn == PieceColor::White {
                            to << 8
                        } else {
                            to >> 8
                        };

                        let attacker = self.get_attackers(king, self.turn.opposite());

                        if attacker & captured != 0 { return !self.is_pinned(from); }
                    }

                    // when a piece is pinned, its not able to block a check, no matter how the position looks
                    !self.is_pinned(from) && self.attacks.is_between(to, self.get_attackers(king, self.turn.opposite()), king)
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
            
            let to = get_to(*m);
            let from = get_from(*m);

            if is_en_passant(*m) {
                // handle phantom pins
                let line = self.attacks.get_line_mask(from, king);
                let ray = self.attacks.get_ray(from, king);

                if line == 0 { return true; }
                
                let captured = if self.turn == PieceColor::White {
                    to << 8
                } else {
                    to >> 8
                };

                // a piece is blocking the pin
                if ray & !from & !king & !captured & self.bb.pieces != 0 { return true; }

                let occupancy = self.bb.pieces & !captured;

                let complement = line & !ray & !from & !king;
                        
                let bishop_attackers = self.magic.get_bishop_moves(from.trailing_zeros() as usize, occupancy);
                let rook_attackers = self.magic.get_rook_moves(from.trailing_zeros() as usize, occupancy);
        
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

            let pin = self.get_pin(from);

            if pin != 0 {
                let line = self.attacks.get_line_mask(from, king);

                to & line != 0
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