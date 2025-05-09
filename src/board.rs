use std::sync::Arc;

use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::{bitboard::Bitboard, moves::{magic::Magic, tables::AttackTables, Position}, piece::{Piece, PieceColor, PieceType}};

#[derive(Debug, Clone, Copy)]
pub struct Castling {
    pub white: (bool, bool),
    pub black: (bool, bool)
}

impl Castling {
    pub fn can_castle_ks(&self, color: PieceColor) -> bool {
        match color {
            PieceColor::White => self.white.0,
            PieceColor::Black => self.black.0
        }
    }

    pub fn can_castle_qs(&self, color: PieceColor) -> bool {
        match color {
            PieceColor::White => self.white.1,
            PieceColor::Black => self.black.1
        }
    }
}


pub struct Board {
    pub bb: Bitboard,
    pub turn: PieceColor,
    pub moves: u32,
    pub halfmove_clock: u32,
    pub target_square: u64,
    pub hash: i64,
    pub hash_table: [i64; 781],
    pub castling: Castling,
    pub magic: Arc<Magic>,
    pub attacks: Arc<AttackTables>,
}

impl Board {
    pub fn new(magic: Arc<Magic>, attacks: Arc<AttackTables>) -> Self {
        Board {
            bb: Bitboard::new(),
            turn: PieceColor::White,
            moves: 1,
            halfmove_clock: 0,
            target_square: 0,
            hash: 0,
            hash_table: [0; 781],
            castling: Castling {
                white: (true, true),
                black: (true, true)
            },
            magic,
            attacks
        }
    }

    pub fn from_fen(fen: &str, magic: Arc<Magic>, attacks: Arc<AttackTables>) -> Self {
        let mut board = Board::new(magic, attacks);
        let parts: Vec<&str> = fen.split_whitespace().collect();
        let position = parts[0];
        let turn = parts[1];
        let c = parts[2];
        let target_square = parts[3];
        let halfmoves = parts[4];
        let moves = parts[5];

        let ranks: Vec<&str> = position.split('/').collect();

        for (j, rank) in ranks.iter().enumerate() {
            let mut i = 0;
            for char in rank.chars().into_iter() {
                if char.is_ascii_digit() {
                    i += char.to_digit(10).unwrap() as usize - 1;
                } else {
                    let color = if "PNBRQK".contains(char) {
                        PieceColor::White
                    } else {
                        PieceColor::Black
                    };

                    let piece_type = match char.to_ascii_lowercase() {
                        'p' => PieceType::Pawn,
                        'n' => PieceType::Knight,
                        'b' => PieceType::Bishop,
                        'r' => PieceType::Rook,
                        'q' => PieceType::Queen,
                        'k' => PieceType::King,
                        a => panic!("Invalid piece type: expected one of PNBRQK, found {}", a)
                    };

                    let piece = Piece {
                        color,
                        piece_type
                    };

                    board.bb.add_piece(piece, Position::bitboard(i, j));
                }
                i += 1;
            }
        }

        if board.bb.white_king == 0 || board.bb.black_king == 0 {
            panic!("Invalid position: no king")
        }

        board.turn = if turn == "b" { PieceColor::Black } else { PieceColor::White };
        board.halfmove_clock = halfmoves.parse().unwrap_or(0);
        board.moves = moves.parse().unwrap_or(1);

        board.castling.white = (c.contains('K'), c.contains('Q'));
        board.castling.black = (c.contains('k'), c.contains('q'));

        if target_square.len() > 0 && target_square != "-" {
            board.target_square = Position {
                x: "abcdefgh".find(target_square.chars().next().unwrap()).unwrap(),
                y: 8 - target_square[1..].parse::<usize>().unwrap()
            }.to_bitboard();
        }

        board.gen_hash();

        board
    }

    pub fn startpos(magic: Arc<Magic>, attacks: Arc<AttackTables>) -> Self {
        Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", magic, attacks)
    }

    pub fn gen_hash(&mut self) {
        let mut hash_array = [0; 781];
        let mut hash = i64::MAX;

        let mut rng = StdRng::seed_from_u64(247);

        for i in 0..(64 * 12 + 4 + 1 + 8) {
            hash_array[i] = rng.random::<i64>();
        }

        for square in 0..64 {
            if let Some(piece) = self.bb.get_piece_at(square) {
                let pos = Position::from_bitboard(square);
                hash ^= hash_array[piece.index() * 64 + pos.y * 8 + pos.x];
            }
        }

        if self.castling.white.0 { hash ^= hash_array[12 * 64]; }
        if self.castling.white.1 { hash ^= hash_array[12 * 64 + 1]; }
        if self.castling.black.0 { hash ^= hash_array[12 * 64 + 2]; }
        if self.castling.black.1 { hash ^= hash_array[12 * 64 + 3]; }

        if self.turn == PieceColor::White {
            hash ^= hash_array[12 * 64 + 4];
        } else {
            hash ^= hash_array[12 * 64 + 5];
        }

        if self.target_square != 0 {
            let pos = Position::from_bitboard(self.target_square);
            hash ^= hash_array[12 * 64 + 4 + 2 + pos.y];
        }

        self.hash = hash;
        self.hash_table = hash_array;
    }
}