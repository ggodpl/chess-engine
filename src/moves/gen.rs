use crate::{bitboard::{AB_FILE_INV, A_FILE_INV, GH_FILE_INV, H_FILE_INV, RANK_1, RANK_2, RANK_7, RANK_8}, board::Board, piece::{Piece, PieceColor, PieceType}};

use super::Move;

impl Board {
    pub fn get_legal_moves(&self) -> Vec<Move> {
        let mut moves = self.get_pseudo_legal_moves();
        self.filter_legal_moves(&mut moves);
        
        moves
    }

    pub fn get_pseudo_legal_moves(&self) -> Vec<Move> {
        let pieces = if self.turn == PieceColor::White {
            self.bb.white_pieces
        } else {
            self.bb.black_pieces
        };

        let mut moves: Vec<Move> = Vec::with_capacity(218);

        let mut rem = pieces;
        while rem != 0 {
            let index = rem.trailing_zeros() as usize;
            let square = 1u64 << index;

            if let Some(piece) = self.bb.get_piece_at(square) {
                match piece.piece_type {
                    PieceType::Pawn => self.add_pawn_moves(piece, square, &mut moves),
                    PieceType::Knight => self.add_knight_moves(piece, square, &mut moves),
                    PieceType::Bishop => self.add_bishop_moves(piece, square, &mut moves),
                    PieceType::Rook => self.add_rook_moves(piece, square, &mut moves),
                    PieceType::Queen => self.add_queen_moves(piece, square, &mut moves),
                    PieceType::King => self.add_king_moves(piece, square, &mut moves),
                }
            }

            rem &= rem - 1;
        }

        moves
    }

    pub fn add_pawn_moves(&self, piece: Piece, square: u64, moves: &mut Vec<Move>) {
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

        let enemy = if piece.color == PieceColor::White {
            self.bb.black_pieces
        } else {
            self.bb.white_pieces
        };

        let captures_mask = if piece.color == PieceColor::White {
            ((square >> 9) & H_FILE_INV) | ((square >> 7) & A_FILE_INV)
        } else {
            ((square << 9) & A_FILE_INV) | ((square << 7) & H_FILE_INV)
        };

        mask |= captures_mask & enemy;

        mask |= if self.target_square != 0 {
            captures_mask & self.target_square
        } else {
            0
        };

        let mut rem = mask;
        while rem != 0 {
            let index = rem.trailing_zeros() as usize;
            let to = 1u64 << index;

            let is_capture = to & enemy != 0;
            let is_en_passant = to & self.target_square != 0;

            let is_promotion = if piece.color == PieceColor::White {
                to & RANK_8 != 0
            } else {
                to & RANK_1 != 0
            };

            if is_promotion {
                for piece_type in [PieceType::Queen, PieceType::Rook, PieceType::Bishop, PieceType::Knight] {
                    moves.push(Move {
                        from: square,
                        to,
                        captured: self.bb.get_piece_at(to),
                        is_capture,
                        is_en_passant,
                        is_castling: false,
                        promotion: Some(piece_type),
                        is_promotion,
                        piece_type: piece.piece_type,
                        color: piece.color,
                    });
                }
            } else {
                moves.push(Move {
                    from: square,
                    to,
                    captured: if is_en_passant {
                        self.bb.get_piece_at(if piece.color == PieceColor::White {
                            to << 8
                        } else {
                            to >> 8
                        })
                    } else {
                        self.bb.get_piece_at(to)
                    },
                    is_capture,
                    is_en_passant,
                    is_castling: false,
                    promotion: None,
                    is_promotion,
                    piece_type: piece.piece_type,
                    color: piece.color,
                });
            }

            rem &= rem - 1;
        } 
    }
    
    pub fn add_knight_moves(&self, piece: Piece, square: u64, moves: &mut Vec<Move>) {
        let knight_moves = ((square << 17) & A_FILE_INV) |
            ((square << 15) & H_FILE_INV) |
            ((square << 10) & AB_FILE_INV) |
            ((square >> 6) & AB_FILE_INV) |
            ((square >> 15) & A_FILE_INV) |
            ((square >> 17) & H_FILE_INV) |
            ((square << 6) & GH_FILE_INV) |
            ((square >> 10) & GH_FILE_INV);
        
        let enemy = if piece.color == PieceColor::White {
            self.bb.black_pieces
        } else {
            self.bb.white_pieces
        };

        let mask = knight_moves & (self.bb.empty | enemy);

        self.add_bitboard_moves(mask, enemy, square, moves, piece);
    }

    pub fn add_king_moves(&self, piece: Piece, square: u64, moves: &mut Vec<Move>) {
        let king_moves = ((square << 1) & A_FILE_INV) |
            ((square >> 1) & H_FILE_INV) |
            (square << 8) |
            (square >> 8) |
            ((square << 9) & A_FILE_INV) |
            ((square << 7) & H_FILE_INV) |
            ((square >> 7) & A_FILE_INV) |
            ((square >> 9) & H_FILE_INV);

        let enemy = if piece.color == PieceColor::White {
            self.bb.black_pieces
        } else {
            self.bb.white_pieces
        };

        let mask = king_moves & (self.bb.empty | enemy);

        self.add_bitboard_moves(mask, enemy, square, moves, piece);

        let color = piece.color.opposite();

        if self.castling.can_castle_ks(piece.color)
            && !self.is_attacked(square, color)
            && !self.is_attacked(square << 1, color) && self.is_empty(square << 1)
            && !self.is_attacked(square << 2, color) && self.is_empty(square << 2) {
            moves.push(Move {
                from: square,
                to: square << 2,
                promotion: None,
                captured: None,
                is_castling: true,
                is_capture: false,
                is_en_passant: false,
                is_promotion: false,
                piece_type: piece.piece_type,
                color: piece.color,
            });
        }

        if self.castling.can_castle_qs(piece.color)
            && !self.is_attacked(square, color)
            && !self.is_attacked(square >> 1, color) && self.is_empty(square >> 1)
            && !self.is_attacked(square >> 2, color) && self.is_empty(square >> 2)
            && self.is_empty(square >> 3) {
            moves.push(Move {
                from: square,
                to: square >> 2,
                promotion: None,
                captured: None,
                is_castling: true,
                is_capture: false,
                is_en_passant: false,
                is_promotion: false,
                piece_type: piece.piece_type,
                color: piece.color,
            });
        }
    }

    pub fn add_bishop_moves(&self, piece: Piece, square: u64, moves: &mut Vec<Move>) {
        let mask = self.magic.get_bishop_moves(square.trailing_zeros() as usize, self.bb.pieces);

        let enemy = if piece.color == PieceColor::White {
            self.bb.black_pieces
        } else {
            self.bb.white_pieces
        };

        let mask = mask & (self.bb.empty | enemy);

        self.add_sliding_moves(piece, mask, square, moves);
    }

    pub fn add_rook_moves(&self, piece: Piece, square: u64, moves: &mut Vec<Move>) {
        let mask = self.magic.get_rook_moves(square.trailing_zeros() as usize, self.bb.pieces);
        
        let enemy = if piece.color == PieceColor::White {
            self.bb.black_pieces
        } else {
            self.bb.white_pieces
        };

        let mask = mask & (self.bb.empty | enemy);

        self.add_sliding_moves(piece, mask, square, moves);
    }

    pub fn add_queen_moves(&self, piece: Piece, square: u64, moves: &mut Vec<Move>) {
        let mask = self.magic.get_queen_moves(square.trailing_zeros() as usize, self.bb.pieces);

        let enemy = if piece.color == PieceColor::White {
            self.bb.black_pieces
        } else {
            self.bb.white_pieces
        };

        let mask = mask & (self.bb.empty | enemy);

        self.add_sliding_moves(piece, mask, square, moves);
    }

    pub fn add_sliding_moves(&self, piece: Piece, mask: u64, square: u64, moves: &mut Vec<Move>) {
        let enemy = if piece.color == PieceColor::White {
            self.bb.black_pieces
        } else {
            self.bb.white_pieces
        };

        self.add_bitboard_moves(mask, enemy, square, moves, piece);
    }

    pub fn add_bitboard_moves(&self, mask: u64, enemy: u64, square: u64, moves: &mut Vec<Move>, piece: Piece) {
        let mut rem = mask;
        while rem != 0 {
            let index = rem.trailing_zeros() as usize;
            let to = 1u64 << index;

            let is_capture = to & enemy != 0;
            
            moves.push(Move { 
                from: square, 
                to,
                is_capture,
                captured: self.bb.get_piece_at(to),
                is_castling: false,
                is_en_passant: false,
                is_promotion: false,
                promotion: None,
                piece_type: piece.piece_type,
                color: piece.color,
            });

            rem &= rem - 1;
        }
    }
}