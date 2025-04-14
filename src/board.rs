use std::{fmt::Display, i64, sync::Arc};

use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::{bitboard::Bitboard, moves::{magic::Magic, Position}, piece::{Piece, PieceColor, PieceType}};

pub struct Castling {
    white: (bool, bool),
    black: (bool, bool)
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
}

impl Board {
    pub fn new(magic: Arc<Magic>) -> Self {
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
        }
    }

    pub fn from_fen(fen: &str, magic: Arc<Magic>) -> Self {
        let mut board = Board::new(magic);
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
                if char.is_digit(10) {
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

                    board.bb.add_piece(piece, Position { x: i, y: j });
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

        board.castling.white = (c.contains("K"), c.contains("Q"));
        board.castling.black = (c.contains("k"), c.contains("q"));

        if target_square.len() > 0 && target_square != "-" {
            board.target_square = Position {
                x: "abcdefgh".find(target_square.chars().next().unwrap()).unwrap(),
                y: 8 - target_square[1..].parse::<usize>().unwrap()
            }.to_bitboard();
        }

        board
    }

    pub fn gen_hash(&mut self) {
        let mut hash_array = [0; 781];
        let mut hash = i64::MAX;

        let mut rng = StdRng::seed_from_u64(247);

        for i in 0..(64 * 12 + 4 + 2 + 8) {
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

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "  ")?;
        for i in 0..8 {
            write!(f, "{} ", "abcdefgh".chars().nth(i).unwrap())?;
        }
        write!(f, "\n")?;
        for rank in 0..8 {
            write!(f, "{} ", 8 - rank)?;
            for file in 0..8 {
                let piece = self.bb.get_piece_at(Position::bitboard(file, rank));
                if let Some(piece) = piece {
                    let piece_char = match piece.piece_type {
                        PieceType::Pawn => "p",
                        PieceType::Knight => "n",
                        PieceType::Bishop => "b",
                        PieceType::Rook => "r",
                        PieceType::Queen => "q",
                        PieceType::King => "k"
                    };
                    
                    write!(f, "{} ", if piece.color == PieceColor::White {
                        piece_char.to_uppercase()
                    } else {
                        piece_char.to_owned()
                    })?;
                } else {
                    write!(f, ". ")?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}