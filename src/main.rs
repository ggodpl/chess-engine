use std::sync::Arc;

use mchess::{board::Board, moves::{magic::Magic, tables::AttackTables}};

fn main() {
    let magic = Arc::new(Magic::new());
    let attacks = Arc::new(AttackTables::new());

    let board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", magic.clone(), attacks.clone());

    println!("{}", board);
}