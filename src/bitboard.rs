use crate::{moves::Position, piece::{Piece, PieceColor, PieceType}};

#[derive(Debug, Clone, Copy)]
pub struct Bitboard {
    pub pieces: u64,
    pub empty: u64,

    pub white_pieces: u64,
    pub black_pieces: u64,
    
    pub white_king: u64,
    pub black_king: u64,
    pub white_queens: u64,
    pub black_queens: u64,
    pub white_rooks: u64,
    pub black_rooks: u64,
    pub white_bishops: u64,
    pub black_bishops: u64,
    pub white_knights: u64,
    pub black_knights: u64,
    pub white_pawns: u64,
    pub black_pawns: u64,

    pub white_attacks: u64,
    pub black_attacks: u64,
}

impl Bitboard {
    pub fn new() -> Self {
        Bitboard {
            pieces: 0,
            empty: !0,
            white_pieces: 0,
            black_pieces: 0,
            white_king: 0,
            black_king: 0,
            white_queens: 0,
            black_queens: 0,
            white_rooks: 0,
            black_rooks: 0,
            white_bishops: 0,
            black_bishops: 0,
            white_knights: 0,
            black_knights: 0,
            white_pawns: 0,
            black_pawns: 0,
            white_attacks: 0,
            black_attacks: 0,
        }
    }

    pub fn add_piece(&mut self, piece: Piece, square: u64) {
        match (&piece.piece_type, &piece.color) {
            (PieceType::Pawn, PieceColor::White) => self.white_pawns |= square,
            (PieceType::Knight, PieceColor::White) => self.white_knights |= square,
            (PieceType::Bishop, PieceColor::White) => self.white_bishops |= square,
            (PieceType::Rook, PieceColor::White) => self.white_rooks |= square,
            (PieceType::Queen, PieceColor::White) => self.white_queens |= square,
            (PieceType::King, PieceColor::White) => self.white_king |= square,
            (PieceType::Pawn, PieceColor::Black) => self.black_pawns |= square,
            (PieceType::Knight, PieceColor::Black) => self.black_knights |= square,
            (PieceType::Bishop, PieceColor::Black) => self.black_bishops |= square,
            (PieceType::Rook, PieceColor::Black) => self.black_rooks |= square,
            (PieceType::Queen, PieceColor::Black) => self.black_queens |= square,
            (PieceType::King, PieceColor::Black) => self.black_king |= square,
        }

        if piece.color == PieceColor::White {
            self.white_pieces |= square;
        } else {
            self.black_pieces |= square;
        }

        self.pieces |= square;
        self.empty &= !square;
    }

    pub fn remove_piece(&mut self, piece: Piece, square: u64) {
        let inv_square = !square;

        match (&piece.piece_type, &piece.color) {
            (PieceType::Pawn, PieceColor::White) => self.white_pawns &= inv_square,
            (PieceType::Knight, PieceColor::White) => self.white_knights &= inv_square,
            (PieceType::Bishop, PieceColor::White) => self.white_bishops &= inv_square,
            (PieceType::Rook, PieceColor::White) => self.white_rooks &= inv_square,
            (PieceType::Queen, PieceColor::White) => self.white_queens &= inv_square,
            (PieceType::King, PieceColor::White) => self.white_king &= inv_square,
            (PieceType::Pawn, PieceColor::Black) => self.black_pawns &= inv_square,
            (PieceType::Knight, PieceColor::Black) => self.black_knights &= inv_square,
            (PieceType::Bishop, PieceColor::Black) => self.black_bishops &= inv_square,
            (PieceType::Rook, PieceColor::Black) => self.black_rooks &= inv_square,
            (PieceType::Queen, PieceColor::Black) => self.black_queens &= inv_square,
            (PieceType::King, PieceColor::Black) => self.black_king &= inv_square,
        }

        if piece.color == PieceColor::White {
            self.white_pieces &= inv_square;
        } else {
            self.black_pieces &= inv_square;
        }

        self.pieces &= inv_square;
        self.empty |= square;
    }

    pub fn remove_piece_at(&mut self, square: u64) {
        let piece = self.get_piece_at(square);

        if let Some(piece) = piece {
            self.remove_piece(piece, square);
        }
    }

    #[inline]
    pub fn get_piece_at(&self, square: u64) -> Option<Piece> {
        if square == 0 { return None; }

        if square & self.white_pawns != 0 { Some(Piece { color: PieceColor::White, piece_type: PieceType::Pawn }) }
        else if square & self.white_knights != 0 { Some(Piece { color: PieceColor::White, piece_type: PieceType::Knight }) }
        else if square & self.white_bishops != 0 { Some(Piece { color: PieceColor::White, piece_type: PieceType::Bishop }) }
        else if square & self.white_rooks != 0 { Some(Piece { color: PieceColor::White, piece_type: PieceType::Rook }) }
        else if square & self.white_queens != 0 { Some(Piece { color: PieceColor::White, piece_type: PieceType::Queen }) }
        else if square & self.white_king != 0 { Some(Piece { color: PieceColor::White, piece_type: PieceType::King }) }
        else if square & self.black_pawns != 0 { Some(Piece { color: PieceColor::Black, piece_type: PieceType::Pawn }) }
        else if square & self.black_knights != 0 { Some(Piece { color: PieceColor::Black, piece_type: PieceType::Knight }) }
        else if square & self.black_bishops != 0 { Some(Piece { color: PieceColor::Black, piece_type: PieceType::Bishop }) }
        else if square & self.black_rooks != 0 { Some(Piece { color: PieceColor::Black, piece_type: PieceType::Rook }) }
        else if square & self.black_queens != 0 { Some(Piece { color: PieceColor::Black, piece_type: PieceType::Queen }) }
        else if square & self.black_king != 0 { Some(Piece { color: PieceColor::Black, piece_type: PieceType::King }) }
        else { None }
    }

    pub fn is_empty(&self, pos: Position) -> bool {
        let square = pos.to_bitboard();

        self.empty & square != 0
    }

    pub fn move_piece(&mut self, square: u64, to: u64) {
        let piece = self.get_piece_at(square);

        if let Some(piece) = piece {
            self.remove_piece(piece, square);
            self.add_piece(piece, to);
        }
    }

    pub fn count_bishops(&self, white: bool) -> u32 {
        if white {
            self.white_bishops.count_ones()
        } else {
            self.black_bishops.count_ones()
        }
    }

    pub fn count_knights(&self, white: bool) -> u32 {
        if white {
            self.white_knights.count_ones()
        } else {
            self.black_knights.count_ones()
        }
    }

    pub fn count_non_bk(&self) -> u32 {
        let material = self.white_pawns | self.white_rooks | self.white_queens | self.black_pawns | self.black_rooks | self.black_queens;
        material.count_ones()
    }

    pub fn count_material(&self, color: PieceColor) -> u32 {
        if color == PieceColor::White {
            self.white_pawns.count_ones()   * PieceType::Pawn.value()   +
            self.white_bishops.count_ones() * PieceType::Bishop.value() +
            self.white_knights.count_ones() * PieceType::Knight.value() +
            self.white_rooks.count_ones()   * PieceType::Rook.value()   +
            self.white_queens.count_ones()  * PieceType::Queen.value()
        } else {
            self.black_pawns.count_ones()   * PieceType::Pawn.value()   +
            self.black_bishops.count_ones() * PieceType::Bishop.value() +
            self.black_knights.count_ones() * PieceType::Knight.value() +
            self.black_rooks.count_ones()   * PieceType::Rook.value()   +
            self.black_queens.count_ones()  * PieceType::Queen.value()
        }
    }
}

pub const RANK_1: u64 = 0xFF00000000000000;
pub const RANK_2: u64 = 0x00FF000000000000;
pub const RANK_3: u64 = 0x0000FF0000000000;
pub const RANK_4: u64 = 0x000000FF00000000;
pub const RANK_5: u64 = 0x00000000FF000000;
pub const RANK_6: u64 = 0x0000000000FF0000;
pub const RANK_7: u64 = 0x000000000000FF00;
pub const RANK_8: u64 = 0x00000000000000FF;

pub const A_FILE: u64 = 0x0101010101010101;
pub const B_FILE: u64 = 0x0202020202020202;
pub const C_FILE: u64 = 0x0404040404040404;
pub const D_FILE: u64 = 0x0808080808080808;
pub const E_FILE: u64 = 0x1010101010101010;
pub const F_FILE: u64 = 0x2020202020202020;
pub const G_FILE: u64 = 0x4040404040404040;
pub const H_FILE: u64 = 0x8080808080808080;

pub const A_FILE_INV: u64 = 0xFEFEFEFEFEFEFEFE;
pub const AB_FILE_INV: u64 = 0xFCFCFCFCFCFCFCFC;
pub const GH_FILE_INV: u64 = 0x3F3F3F3F3F3F3F3F;
pub const H_FILE_INV: u64 = 0x7F7F7F7F7F7F7F7F;

pub const A8: u64 = 0x1;
pub const H8: u64 = 0x80;
pub const A1: u64 = 0x100000000000000;
pub const H1: u64 = 0x8000000000000000;

pub const COLOR_MASK: u64 = 0xAA55AA55AA55AA55;