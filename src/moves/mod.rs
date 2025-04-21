use crate::{board::Board, piece::{Piece, PieceColor, PieceType}};

pub mod magic;
pub mod values;
pub mod gen;
pub mod tables;
pub mod legal;
pub mod make;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize
}

impl Position {
    pub fn bitboard(x: usize, y: usize) -> u64 {
        1u64 << (x + y * 8)
    }
    
    pub fn to_bitboard(&self) -> u64 {
        Position::bitboard(self.x, self.y)
    }

    pub fn from_bitboard(square: u64) -> Self {
        let index = square.trailing_zeros() as usize;
        Position { x: index % 8, y: index / 8 }
    }

    pub fn to_vector(&self) -> Vector {
        Vector {
            x: self.x as i32,
            y: self.y as i32
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub x: i32,
    pub y: i32
}

impl Vector {
    pub fn between(pos1: Position, pos2: Position) -> Self {
        let vec1 = pos1.to_vector();
        let vec2 = pos2.to_vector();
        let x_diff = vec2.x - vec1.x;
        let y_diff = vec2.y - vec1.y;

        Vector {
            x: x_diff.signum(),
            y: y_diff.signum()
        }
    }

    pub fn inv(&self) -> Self {
        Vector {
            x: -self.x,
            y: -self.y
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Move {
    pub from: u64,
    pub to: u64,
    pub promotion: Option<PieceType>,
    pub captured: Option<Piece>,
    pub is_castling: bool,
    pub is_en_passant: bool,
    pub is_capture: bool,
    pub is_promotion: bool,
    pub piece_type: PieceType,
    pub color: PieceColor,
}

impl Board {
    pub fn get_attackers(&self, square: u64, color: PieceColor) -> u64 {
        if square == 0 { return 0; }
        let mut mask = 0;

        let index = square.trailing_zeros() as usize;

        let pawns = if color == PieceColor::White { self.bb.white_pawns } else { self.bb.black_pawns };
        let knights = if color == PieceColor::White { self.bb.white_knights } else { self.bb.black_knights };
        let bishops = if color == PieceColor::White { self.bb.white_bishops } else { self.bb.black_bishops };
        let rooks = if color == PieceColor::White { self.bb.white_rooks } else { self.bb.black_rooks };
        let queens = if color == PieceColor::White { self.bb.white_queens } else { self.bb.black_queens };
        let king = if color == PieceColor::White { self.bb.white_king } else { self.bb.black_king };

        mask |= self.attacks.pawn_attacks[color.opposite().index()][index] & pawns;
        mask |= self.attacks.knight_attacks[index] & knights;
        mask |= self.attacks.king_attacks[index] & king;

        let bishop_attackers = bishops | queens;

        mask |= self.magic.get_bishop_moves(index, self.bb.pieces) & bishop_attackers;
        
        let rook_attackers = rooks | queens;
        
        mask |= self.magic.get_rook_moves(index, self.bb.pieces) & rook_attackers;

        mask
    }

    pub fn is_attacked(&self, square: u64, color: PieceColor) -> bool {
        self.get_attackers(square, color) != 0
    }

    pub fn is_empty(&self, square: u64) -> bool {
        square & self.bb.empty != 0
    }

    pub fn is_checked(&self, color: PieceColor) -> bool {
        let king = if color == PieceColor::White {
            self.bb.white_king
        } else {
            self.bb.black_king
        };

        self.is_attacked(king, color.opposite())
    }

    pub fn is_double_checked(&self, color: PieceColor) -> bool {
        let king = if color == PieceColor::White {
            self.bb.white_king
        } else {
            self.bb.black_king
        };

        self.get_attackers(king, color).count_ones() >= 2
    }
}