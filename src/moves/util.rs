use crate::{bitboard::COLOR_MASK, board::Board, piece::{PieceColor, PieceType}};

use super::{helper::{create, to_move_type}, Move, Position};

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

        let opposite_king = if color == PieceColor::White { self.bb.black_king } else { self.bb.white_king };

        mask |= self.attacks.pawn_attacks[color.opposite().index()][index] & pawns;
        mask |= self.attacks.knight_attacks[index] & knights;
        mask |= self.attacks.king_attacks[index] & king;

        let bishop_attackers = bishops | queens;

        mask |= self.magic.get_bishop_moves(index, self.bb.pieces & !opposite_king) & bishop_attackers;
        
        let rook_attackers = rooks | queens;
        
        mask |= self.magic.get_rook_moves(index, self.bb.pieces & !opposite_king) & rook_attackers;

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

        self.get_attackers(king, color.opposite()).count_ones() >= 2
    }

    pub fn check_insufficient_material(&self) -> bool {
        let no_heavy_pieces = self.bb.count_non_bk() == 0;
        let white_no_minor = (self.bb.white_bishops | self.bb.white_knights).count_ones() == 0;
        let black_no_minor = (self.bb.black_bishops | self.bb.black_knights).count_ones() == 0;

        let white_one_bishop = self.bb.count_bishops(true) == 1 && self.bb.count_knights(true) == 0;
        let white_one_knight = self.bb.count_bishops(true) == 0 && self.bb.count_knights(true) == 1;

        let black_one_bishop = self.bb.count_bishops(false) == 1 && self.bb.count_knights(false) == 0;
        let black_one_knight = self.bb.count_bishops(false) == 0 && self.bb.count_knights(false) == 1;

        no_heavy_pieces && (
            (white_no_minor && (
                black_no_minor ||
                black_one_bishop ||
                black_one_knight
            )) ||
            (black_no_minor && (
                white_one_bishop ||
                white_one_knight
            )) ||
            white_one_bishop && black_one_bishop && self.bb.white_bishops & COLOR_MASK == self.bb.black_bishops & COLOR_MASK
        )
    }

    pub fn is_checkmate(&self) -> bool {
        self.is_checked(self.turn) && self.get_legal_moves().is_empty()
    }

    pub fn is_draw(&self) -> bool {
        self.check_insufficient_material() // insufficient material
        || self.halfmove_clock > 100 // 50-move rule
    }

    pub fn parse_uci_string(&self, string: &str) -> Option<Move> {
        if string.len() < 4 {
            return None;
        }

        let chars: Vec<char> = string.chars().collect();

        if !(matches!(chars[0], 'a'..='h')
            && matches!(chars[1], '1'..='8')
            && matches!(chars[2], 'a'..='h')
            && matches!(chars[3], '1'..='8')) {
            return None;
        }

        let from_file = (chars[0] as u8 - b'a') as usize;
        let from_rank = 8 - (chars[1] as u8 - b'0') as usize;
        let to_file = (chars[2] as u8 - b'a') as usize;
        let to_rank = 8 - (chars[3] as u8 - b'0') as usize;

        let from = Position::bitboard(from_file, from_rank);
        let to = Position::bitboard(to_file, to_rank);

        let promotion = if chars.len() > 4 {
            match chars[4] {
                'q' => Some(PieceType::Queen),
                'r' => Some(PieceType::Rook),
                'b' => Some(PieceType::Bishop),
                'n' => Some(PieceType::Knight),
                _ => None
            }
        } else { None };

        let piece = self.bb.get_piece_at(from);

        let piece = if let Some(piece) = piece {
            piece
        } else {
            return None;
        };

        let captured = self.bb.get_piece_at(to);

        let is_castling = piece.piece_type == PieceType::King && (from >> 2 == to || from << 2 == to);
        let is_en_passant = to & self.target_square != 0;

        Some(create(from, to, promotion, to_move_type(captured.is_some(), is_castling, is_en_passant), piece.piece_type, piece.color))
    }
}