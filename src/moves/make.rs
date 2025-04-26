use crate::{bitboard::{Bitboard, A1, A8, H1, H8, RANK_2, RANK_4, RANK_5, RANK_7}, board::{Board, Castling}, piece::{Piece, PieceColor, PieceType}};

use super::Move;

#[derive(Debug, Clone, Copy)]
pub struct Meta {
    pub turn: PieceColor,
    pub moves: u32,
    pub halfmove_clock: u32,
    pub target_square: u64,
    pub castling: Castling,
}

pub type State = (Meta, Bitboard);

impl Board {
    pub fn make_move(&mut self, m: &Move) -> State {
        let meta = Meta {
            turn: self.turn,
            moves: self.moves,
            halfmove_clock: self.halfmove_clock,
            target_square: self.target_square,
            castling: self.castling
        };

        let bb = self.bb.clone();

        // capture
        if let Some(captured) = self.bb.get_piece_at(m.to) {
            if captured.piece_type == PieceType::Rook {
                match m.to {
                    A1 => self.castling.white.1 = false,
                    H1 => self.castling.white.0 = false,
                    A8 => self.castling.black.1 = false,
                    H8 => self.castling.black.0 = false,
                    _ => {}
                }
            }
        }
        self.bb.remove_piece_at(m.to);

        self.bb.move_piece(m.from, m.to);

        if m.is_promotion {
            self.bb.remove_piece_at(m.to);
            self.bb.add_piece(Piece { color: m.color, piece_type: m.promotion.unwrap() }, m.to);
        }

        if m.is_en_passant {
            self.bb.remove_piece_at(if m.color == PieceColor::White {
                m.to << 8
            } else {
                m.to >> 8
            });
        }

        if m.is_castling {
            let kingside = m.from << 2 == m.to;
            if kingside {
                self.bb.move_piece(m.to << 1, m.to >> 1);
            } else {
                self.bb.move_piece(m.to >> 2, m.to << 1);
            }

            match m.to {
                A1 => self.castling.white.1 = false,
                H1 => self.castling.white.0 = false,
                A8 => self.castling.black.1 = false,
                H8 => self.castling.black.0 = false,
                _ => {}
            }
        }

        if m.piece_type == PieceType::King {
            if m.color == PieceColor::White {
                self.castling.white = (false, false);
            } else {
                self.castling.black = (false, false);
            }
        }

        if m.piece_type == PieceType::Rook {
            if m.color == PieceColor::White {
                match m.from {
                    A1 => self.castling.white.1 = false,
                    H1 => self.castling.white.0 = false,
                    _ => {}
                }
            } else if m.color == PieceColor::Black {
                match m.from {
                    A8 => self.castling.black.1 = false,
                    H8 => self.castling.black.0 = false,
                    _ => {}
                }
            }
        }

        if m.piece_type == PieceType::Pawn {
            if (m.color == PieceColor::White && m.from & RANK_2 != 0 && m.to & RANK_4 != 0) ||
               (m.color == PieceColor::Black && m.from & RANK_7 != 0 && m.to & RANK_5 != 0) {
                self.target_square = if m.color == PieceColor::White {
                    m.to << 8
                } else {
                    m.to >> 8
                };
            } else {
                self.target_square = 0;
            }
        } else {
            self.target_square = 0;
        }

        self.turn = self.turn.opposite();
    
        if m.is_capture || m.is_promotion || m.piece_type == PieceType::Pawn {
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock += 1;
        }

        if self.turn == PieceColor::White {
            self.moves += 1;
        }

        (meta, bb)
    }

    pub fn unmake_move(&mut self, state: &State) {
        let (meta, bb) = state;

        self.turn = meta.turn;
        self.moves = meta.moves;
        self.halfmove_clock = meta.halfmove_clock;
        self.target_square = meta.target_square;
        self.castling = meta.castling;

        self.bb = bb.clone();
    }
}