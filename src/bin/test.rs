use std::sync::Arc;

use mchess::{board::Board, moves::{magic::Magic, tables::AttackTables}, search::Search};

fn main() {
    let magic = Arc::new(Magic::new());
    let attacks = Arc::new(AttackTables::new());

    let mut board = Board::startpos(magic, attacks);
    let mut search = Search::new();

    println!("{}", search.search(&mut board, 6))
}