use std::fmt;

use crate::{board::Board, moves::{helper::{get_from, get_promotion, get_to}, Move, Position}, piece::{PieceColor, PieceType}, search::SearchResult};

impl fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "  ")?;
        for i in 0..8 {
            write!(f, "{} ", "abcdefgh".chars().nth(i).unwrap())?;
        }
        writeln!(f)?;
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
            writeln!(f)?;
        }
        Ok(())
    }
}

pub struct MoveDisplay(pub Move);

impl fmt::Display for MoveDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let m = self.0;

        let promotion = get_promotion(m);
        let from = get_from(m);
        let to = get_to(m);

        let promotion_char = if let Some(piece_type) = promotion {
            match piece_type {
                PieceType::Knight => "n",
                PieceType::Bishop => "b",
                PieceType::Rook => "r",
                PieceType::Queen => "q",
                _ => unreachable!()
            }
        } else {
            ""
        };

        write!(f, "{}{}{}", Position::from_bitboard(from), Position::from_bitboard(to), promotion_char)
    }
}

pub struct MoveList<'a>(pub &'a Vec<Move>);

impl fmt::Display for MoveList<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for &m in self.0 {
            write!(f, "{} ", MoveDisplay(m))?;
        }

        Ok(())
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let file_char = "abcdefgh".chars().nth(self.x).unwrap();

        write!(f, "{}{}", file_char, 8 - self.y)
    }
}

pub fn show_mask(mask: u64) {
    print!("  ");
    for i in 0..8 {
        print!("{} ", "abcdefgh".chars().nth(i).unwrap());
    }
    println!();
    for rank in 0..8 {
        print!("{} ", 8 - rank);
        for file in 0..8 {
            let pos = Position { x: file, y: rank };
            let q = mask & pos.to_bitboard();
            if q == 0 {
                print!(". ");
            } else {
                print!("1 ");
            }
        }
        println!();
    }
}

impl fmt::Display for SearchResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "value: {}, moves: {}", self.value, MoveList(&self.moves))
    }
}