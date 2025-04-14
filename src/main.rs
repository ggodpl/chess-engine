use std::sync::Arc;

use mchess::{board::Board, moves::magic::Magic};

fn main() {
    let magic = Arc::new(Magic::new());
    let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", magic.clone());

    println!("{}", board);
}