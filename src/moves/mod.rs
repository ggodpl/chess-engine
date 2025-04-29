pub mod magic;
pub mod values;
pub mod gen;
pub mod tables;
pub mod legal;
pub mod make;
pub mod util;
pub mod helper;
pub mod attacks;

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
// 0000000000 0 000 00 000 000000 000000
// unused     c pt  t  p   to     from
// p - promotion type
// t - move type
// pt - piece type
// c - piece color
pub type Move = u32;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MoveType {
    Normal,
    Capture,
    Castling,
    EnPassant,
}