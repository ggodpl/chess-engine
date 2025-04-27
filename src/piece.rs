#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceColor {
    White,
    Black
}

impl PieceColor {
    pub fn opposite(&self) -> Self {
        match self {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White
        }
    }
    
    pub fn index(&self) -> usize {
        match self {
            PieceColor::White => 0,
            PieceColor::Black => 1
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

impl PieceType {
    pub fn index(&self) -> usize {
        match self {
            PieceType::Pawn => 0,
            PieceType::Knight => 1,
            PieceType::Bishop => 2,
            PieceType::Rook => 3,
            PieceType::Queen => 4,
            PieceType::King => 5
        }
    }

    pub fn value(&self) -> u32 {
        match self {
            PieceType::Pawn => 1,
            PieceType::Bishop => 3,
            PieceType::Knight => 3,
            PieceType::Rook => 5,
            PieceType::Queen => 9,
            PieceType::King => 100
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub color: PieceColor,
    pub piece_type: PieceType
}

impl Piece {
    pub fn index(&self) -> usize {
        self.piece_type.index() + if self.color == PieceColor::White { 0 } else { 6 }
    }

    pub fn index_from(piece_type: PieceType, color: PieceColor) -> usize {
        piece_type.index() + if color == PieceColor::White { 0 } else { 6 }
    }
}