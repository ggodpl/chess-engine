use crate::piece::{Piece, PieceType};

pub mod magic;
pub mod values;

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
}

#[derive(Debug, Clone, Copy)]
pub struct Move {
    pub from: Position,
    pub to: Position,
    pub promotion: Option<PieceType>,
    pub captured: Option<Piece>,
    pub is_castling: bool,
    pub is_en_passant: bool
}