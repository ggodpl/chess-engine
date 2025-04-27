
use crate::{board::Board, piece::{Piece, PieceColor, PieceType}};

use super::{Move, MoveType};

pub fn create(from: u64, to: u64, promotion: Option<PieceType>, move_type: MoveType, piece_type: PieceType, color: PieceColor) -> Move {
    let from = from.trailing_zeros() as u32;
    let to = to.trailing_zeros() as u32;

    let mut m = from | (to << 6);

    let promotion = match promotion {
        None => 0,
        Some(PieceType::Knight) => 1,
        Some(PieceType::Bishop) => 2,
        Some(PieceType::Rook) => 3,
        Some(PieceType::Queen) => 4,
        _ => 0
    };

    m |= promotion << 12;
    m |= (move_type as u32) << 16;
    m |= (piece_type as u32) << 18;
    m |= (color as u32) << 21;
    m
}

pub fn get_promotion(m: Move) -> Option<PieceType> {
    let promotion = (m & 0x7000) >> 12;

    match promotion {
        1 => Some(PieceType::Knight),
        2 => Some(PieceType::Bishop),
        3 => Some(PieceType::Rook),
        4 => Some(PieceType::Queen),
        _ => None
    }
}

pub fn get_from(m: Move) -> u64 {
    1u64 << (m & 0x3F)
}

pub fn get_to(m: Move) -> u64 {
    1u64 << ((m >> 6) & 0x3F)
}

pub fn get_move_type(m: Move) -> MoveType {
    match (m >> 16) & 0x3 {
        0 => MoveType::Normal,
        1 => MoveType::Capture,
        2 => MoveType::Castling,
        3 => MoveType::EnPassant,
        _ => unreachable!()
    }
}

pub fn get_piece_type(m: Move) -> PieceType {
    match (m >> 18) & 0x7 {
        0 => PieceType::Pawn,
        1 => PieceType::Knight,
        2 => PieceType::Bishop,
        3 => PieceType::Rook,
        4 => PieceType::Queen,
        5 => PieceType::King,
        _ => unreachable!()
    }
}

pub fn get_color(m: Move) -> PieceColor {
    if (m >> 21) & 1 == 0 {
        PieceColor::White
    } else {
        PieceColor::Black
    }
}

pub fn to_move_type(is_capture: bool, is_castling: bool, is_en_passant: bool) -> MoveType {
    if is_capture { return MoveType::Capture; }
    if is_castling { return MoveType::Castling; }
    if is_en_passant { return MoveType::EnPassant; }
    
    MoveType::Normal
}

pub fn is_capture(m: Move) -> bool {
    get_move_type(m) == MoveType::Capture
}

pub fn is_castling(m: Move) -> bool {
    get_move_type(m) == MoveType::Castling
}

pub fn is_en_passant(m: Move) -> bool {
    get_move_type(m) == MoveType::EnPassant
}

pub fn is_promotion(m: Move) -> bool {
    m & 0x7000 != 0
}

pub fn get_captured(m: Move, board: &Board) -> Option<Piece> {
    let to = get_to(m);
    if is_en_passant(m) {
        let captured = if get_color(m) == PieceColor::White {
            to << 8
        } else {
            to >> 8
        };

        return board.bb.get_piece_at(captured);
    }
    board.bb.get_piece_at(to)
}