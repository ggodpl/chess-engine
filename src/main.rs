use std::sync::Arc;

use mchess::{board::Board, evaluation::evaluate, moves::{magic::Magic, tables::AttackTables}, search::Search};

fn main() {
    let magic = Arc::new(Magic::new());
    let attacks = Arc::new(AttackTables::new());

    let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", magic.clone(), attacks.clone());

    println!("{}", board);
    println!("{:?}", board.get_legal_moves());

    println!("{}", evaluate(&board).to_value());

    let mut search = Search::new();
    println!("{:?}", search.search(&mut board, 5));
}