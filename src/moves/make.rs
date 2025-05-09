use crate::{bitboard::{Bitboard, A1, A8, H1, H8, RANK_2, RANK_4, RANK_5, RANK_7}, board::{Board, Castling}, piece::{Piece, PieceColor, PieceType}};

use super::{helper::{get_color, get_from, get_piece_type, get_promotion, get_to, is_capture, is_castling, is_en_passant, is_promotion}, Move, Position};

#[derive(Debug, Clone, Copy)]
pub struct Meta {
    pub turn: PieceColor,
    pub moves: u32,
    pub halfmove_clock: u32,
    pub target_square: u64,
    pub castling: Castling,
}

pub type State = (Meta, Bitboard, i64);

impl Board {
    pub fn update_hash(&mut self, m: Move, state: &State) {
        let piece_type = get_piece_type(m);
        let color = get_color(m);
        let from = get_from(m);
        let to = get_to(m);

        let hash_index = Piece::index_from(piece_type, color);
        let from_pos = Position::from_bitboard(from);
        let to_pos = Position::from_bitboard(to);
        self.hash ^= self.hash_table[hash_index * 64 + from_pos.y * 8 + from_pos.x];
        self.hash ^= self.hash_table[hash_index * 64 + to_pos.y * 8 + to_pos.x];

        if let Some(captured) = state.1.get_piece_at(to) {
            let pos = Position::from_bitboard(to);
            self.hash ^= self.hash_table[captured.index() * 64 + pos.y * 8 + pos.x];
        }

        if is_en_passant(m) {
            let square = if color == PieceColor::White {
                to << 8
            } else {
                to >> 8
            };
            let pos = Position::from_bitboard(square);
            
            self.hash ^= self.hash_table[state.1.get_piece_at(square).unwrap().index() * 64 + pos.y * 8 + pos.x];
        }

        if is_castling(m) {
            let kingside = from << 2 == to;
            let rook_square = if kingside {
                to << 1
            } else {
                to >> 2
            };

            let rook_from = Position::from_bitboard(rook_square);

            let rook_to = Position::from_bitboard(if kingside {
                to >> 1
            } else {
                to << 1
            });

            let rook_index = state.1.get_piece_at(rook_square).unwrap().index();

            self.hash ^= self.hash_table[rook_index * 64 + rook_from.y * 8 + rook_from.x];
            self.hash ^= self.hash_table[rook_index * 64 + rook_to.y * 8 + rook_to.x];

            match to {
                A1 => self.hash ^= self.hash_table[12 * 64 + 1],
                H1 => self.hash ^= self.hash_table[12 * 64],
                A8 => self.hash ^= self.hash_table[12 * 64 + 3],
                H8 => self.hash ^= self.hash_table[12 * 64 + 2],
                _ => {}
            }
        }

        if piece_type == PieceType::King {
            if color == PieceColor::White {
                self.hash ^= self.hash_table[12 * 64];
                self.hash ^= self.hash_table[12 * 64 + 1];
            } else {
                self.hash ^= self.hash_table[12 * 64 + 2];
                self.hash ^= self.hash_table[12 * 64 + 3];
            }
        }

        if piece_type == PieceType::Rook {
            if color == PieceColor::White {
                match from {
                    A1 => self.hash ^= self.hash_table[12 * 64 + 1],
                    H1 => self.hash ^= self.hash_table[12 * 64],
                    _ => {}
                }
            } else if color == PieceColor::Black {
                match from {
                    A8 => self.hash ^= self.hash_table[12 * 64 + 3],
                    H8 => self.hash ^= self.hash_table[12 * 64 + 2],
                    _ => {}
                }
            }
        }

        if state.0.target_square != 0 {
            let pos = Position::from_bitboard(state.0.target_square);
            self.hash ^= self.hash_table[12 * 64 + 5 + pos.x];
        }

        if self.target_square != 0 {
            let pos = Position::from_bitboard(self.target_square);
            self.hash ^= self.hash_table[12 * 64 + 5 + pos.x];
        }

        if is_promotion(m) {
            self.hash ^= self.hash_table[hash_index * 64 + to_pos.y * 8 + to_pos.x];
            let new_index = Piece::index_from(get_promotion(m).unwrap(), color);
            self.hash ^= self.hash_table[new_index * 64 + to_pos.y * 8 + to_pos.x];
        }

        self.hash ^= self.hash_table[12 * 64 + 4];
        self.hash ^= self.hash_table[12 * 64 + 5];
    }

    pub fn make_move(&mut self, m: Move) -> State {
        let meta = Meta {
            turn: self.turn,
            moves: self.moves,
            halfmove_clock: self.halfmove_clock,
            target_square: self.target_square,
            castling: self.castling
        };

        let bb = self.bb;

        let piece_type = get_piece_type(m);
        let color = get_color(m);
        let from = get_from(m);
        let to = get_to(m);

        // capture
        if let Some(captured) = self.bb.get_piece_at(to) {
            if captured.piece_type == PieceType::Rook {
                match to {
                    A1 => self.castling.white.1 = false,
                    H1 => self.castling.white.0 = false,
                    A8 => self.castling.black.1 = false,
                    H8 => self.castling.black.0 = false,
                    _ => {}
                }
            }
        }
        self.bb.remove_piece_at(to);

        self.bb.move_piece(from, to);

        if is_promotion(m) {
            self.bb.remove_piece_at(to);
            self.bb.add_piece(Piece { color, piece_type: get_promotion(m).unwrap() }, to);
        }

        if is_en_passant(m) {
            self.bb.remove_piece_at(if color == PieceColor::White {
                to << 8
            } else {
                to >> 8
            });
        }

        if is_castling(m) {
            let kingside = from << 2 == to;
            if kingside {
                self.bb.move_piece(to << 1, to >> 1);
            } else {
                self.bb.move_piece(to >> 2, to << 1);
            }

            match to {
                A1 => self.castling.white.1 = false,
                H1 => self.castling.white.0 = false,
                A8 => self.castling.black.1 = false,
                H8 => self.castling.black.0 = false,
                _ => {}
            }
        }

        if piece_type == PieceType::King {
            if color == PieceColor::White {
                self.castling.white = (false, false);
            } else {
                self.castling.black = (false, false);
            }
        }

        if piece_type == PieceType::Rook {
            if color == PieceColor::White {
                match from {
                    A1 => self.castling.white.1 = false,
                    H1 => self.castling.white.0 = false,
                    _ => {}
                }
            } else if color == PieceColor::Black {
                match from {
                    A8 => self.castling.black.1 = false,
                    H8 => self.castling.black.0 = false,
                    _ => {}
                }
            }
        }

        if piece_type == PieceType::Pawn {
            if (color == PieceColor::White && from & RANK_2 != 0 && to & RANK_4 != 0) ||
               (color == PieceColor::Black && from & RANK_7 != 0 && to & RANK_5 != 0) {
                self.target_square = if color == PieceColor::White {
                    to << 8
                } else {
                    to >> 8
                };
            } else {
                self.target_square = 0;
            }
        } else {
            self.target_square = 0;
        }

        self.turn = self.turn.opposite();
    
        if is_capture(m) || is_promotion(m) || piece_type == PieceType::Pawn {
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock += 1;
        }

        if self.turn == PieceColor::White {
            self.moves += 1;
        }

        let state = (meta, bb, self.hash);

        self.update_hash(m, &state);

        state
    }

    pub fn unmake_move(&mut self, state: &State) {
        let (meta, bb, hash) = state;

        self.turn = meta.turn;
        self.moves = meta.moves;
        self.halfmove_clock = meta.halfmove_clock;
        self.target_square = meta.target_square;
        self.castling = meta.castling;

        self.bb.clone_from(bb);

        self.hash.clone_from(hash);
    }

    pub fn make_null_move(&mut self) -> (Meta, i64) {
        let meta = Meta {
            turn: self.turn,
            moves: self.moves,
            halfmove_clock: self.halfmove_clock,
            target_square: self.target_square,
            castling: self.castling
        };

        let hash = self.hash;
        
        self.turn = self.turn.opposite();

        if self.target_square != 0 {
            let pos = Position::from_bitboard(self.target_square);
            self.hash ^= self.hash_table[12 * 64 + 5 + pos.x];
            self.target_square = 0;
        }

        self.hash ^= self.hash_table[12 * 64 + 4];
        self.hash ^= self.hash_table[12 * 64 + 5];

        (meta, hash)
    }

    pub fn unmake_null_move(&mut self, state: &(Meta, i64)) {
        let (meta, hash) = state;

        self.turn = meta.turn;
        self.target_square = meta.target_square;

        self.hash.clone_from(hash);
    }
}