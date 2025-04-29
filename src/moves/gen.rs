use crate::{bitboard::{RANK_1, RANK_8}, board::Board, piece::{Piece, PieceColor, PieceType}};

use super::{helper::{create, to_move_type}, Move};

impl Board {
    pub fn get_legal_moves(&mut self) -> Vec<Move> {
        let mut moves = self.get_pseudo_legal_moves(self.turn);
        self.filter_legal_moves(&mut moves);
        
        moves
    }

    pub fn get_pseudo_legal_moves(&mut self, color: PieceColor) -> Vec<Move> {
        self.bb.white_attacks = 0;
        self.bb.black_attacks = 0;

        let pieces = if color == PieceColor::White {
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
                let enemy = if piece.color == PieceColor::White {
                    self.bb.black_pieces
                } else {
                    self.bb.white_pieces
                };

                match piece.piece_type {
                    PieceType::Pawn => self.add_pawn_moves(piece, square, &mut moves),
                    PieceType::Knight => {
                        let mask = self.get_knight_attacks(square, enemy);

                        if piece.color == PieceColor::White {
                            self.bb.white_attacks |= mask;
                        } else {
                            self.bb.black_attacks |= mask;
                        }

                        self.add_bitboard_moves(mask, enemy, square, &mut moves, piece);
                    },
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

    pub(self) fn add_pawn_moves(&mut self, piece: Piece, square: u64, moves: &mut Vec<Move>) {
        let enemy = if piece.color == PieceColor::White {
            self.bb.black_pieces
        } else {
            self.bb.white_pieces
        };

        let mask = self.get_pawn_attacks(square, enemy, piece);

        if piece.color == PieceColor::White {
            self.bb.white_attacks |= mask;
        } else {
            self.bb.black_attacks |= mask;
        }

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
                    moves.push(create(
                        square, 
                        to, 
                        Some(piece_type), 
                        to_move_type(is_capture, false, is_en_passant), 
                        piece.piece_type, 
                        piece.color
                    ));
                }
            } else {
                moves.push(create(
                    square, 
                    to, 
                    None,
                    to_move_type(is_capture, false, is_en_passant),
                    piece.piece_type,
                    piece.color,
                ));
            }

            rem &= rem - 1;
        } 
    }

    pub(self) fn add_king_moves(&mut self, piece: Piece, square: u64, moves: &mut Vec<Move>) {
        let enemy = if piece.color == PieceColor::White {
            self.bb.black_pieces
        } else {
            self.bb.white_pieces
        };

        let mask = self.get_king_attacks(square, enemy);

        if piece.color == PieceColor::White {
            self.bb.white_attacks |= mask;
        } else {
            self.bb.black_attacks |= mask;
        }

        self.add_bitboard_moves(mask, enemy, square, moves, piece);

        let color = piece.color.opposite();

        if self.castling.can_castle_ks(piece.color)
            && !self.is_attacked(square, color)
            && !self.is_attacked(square << 1, color) && self.is_empty(square << 1)
            && !self.is_attacked(square << 2, color) && self.is_empty(square << 2) {
            moves.push(create(
                square,
                square << 2,
                None,
                super::MoveType::Castling,
                piece.piece_type,
                piece.color
            ));
        }

        if self.castling.can_castle_qs(piece.color)
            && !self.is_attacked(square, color)
            && !self.is_attacked(square >> 1, color) && self.is_empty(square >> 1)
            && !self.is_attacked(square >> 2, color) && self.is_empty(square >> 2)
            && self.is_empty(square >> 3) {
            moves.push(create(
                square,
                square >> 2,
                None,
                super::MoveType::Castling,
                piece.piece_type,
                piece.color
            ));
        }
    }

    pub(self) fn add_bishop_moves(&mut self, piece: Piece, square: u64, moves: &mut Vec<Move>) {
        let mask = self.magic.get_bishop_moves(square.trailing_zeros() as usize, self.bb.pieces);
        
        if piece.color == PieceColor::White {
            self.bb.white_attacks |= mask;
        } else {
            self.bb.black_attacks |= mask;
        }

        let enemy = if piece.color == PieceColor::White {
            self.bb.black_pieces
        } else {
            self.bb.white_pieces
        };

        let mask = mask & (self.bb.empty | enemy);

        self.add_sliding_moves(piece, mask, square, moves);
    }

    pub(self) fn add_rook_moves(&mut self, piece: Piece, square: u64, moves: &mut Vec<Move>) {
        let mask = self.magic.get_rook_moves(square.trailing_zeros() as usize, self.bb.pieces);
        
        if piece.color == PieceColor::White {
            self.bb.white_attacks |= mask;
        } else {
            self.bb.black_attacks |= mask;
        }
        
        let enemy = if piece.color == PieceColor::White {
            self.bb.black_pieces
        } else {
            self.bb.white_pieces
        };

        let mask = mask & (self.bb.empty | enemy);

        self.add_sliding_moves(piece, mask, square, moves);
    }

    pub(self) fn add_queen_moves(&mut self, piece: Piece, square: u64, moves: &mut Vec<Move>) {
        let mask = self.magic.get_queen_moves(square.trailing_zeros() as usize, self.bb.pieces);
        
        if piece.color == PieceColor::White {
            self.bb.white_attacks |= mask;
        } else {
            self.bb.black_attacks |= mask;
        }

        let enemy = if piece.color == PieceColor::White {
            self.bb.black_pieces
        } else {
            self.bb.white_pieces
        };

        let mask = mask & (self.bb.empty | enemy);

        self.add_sliding_moves(piece, mask, square, moves);
    }

    pub(self) fn add_sliding_moves(&self, piece: Piece, mask: u64, square: u64, moves: &mut Vec<Move>) {
        let enemy = if piece.color == PieceColor::White {
            self.bb.black_pieces
        } else {
            self.bb.white_pieces
        };

        self.add_bitboard_moves(mask, enemy, square, moves, piece);
    }

    pub(self) fn add_bitboard_moves(&self, mask: u64, enemy: u64, square: u64, moves: &mut Vec<Move>, piece: Piece) {
        let mut rem = mask;
        while rem != 0 {
            let index = rem.trailing_zeros() as usize;
            let to = 1u64 << index;

            let is_capture = to & enemy != 0;
            
            moves.push(create(
                square,
                to,
                None,
                to_move_type(is_capture, false, false),
                piece.piece_type,
                piece.color
            ));

            rem &= rem - 1;
        }
    }
}