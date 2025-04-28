use std::{sync::Arc, time::Instant};

use mchess::{board::Board, moves::{magic::Magic, tables::AttackTables}, search::Search};

fn main() {
    let magic = Arc::new(Magic::new());
    let attacks = Arc::new(AttackTables::new());

    let mut board = Board::startpos(magic, attacks);
    let mut search = Search::new();

    let start = Instant::now();

    println!("{}", search.search(&mut board, 6));

    println!("{:?}", start.elapsed());
    println!("{} {} {}", search.nodes, search.tt_hits, search.tt.len());
}