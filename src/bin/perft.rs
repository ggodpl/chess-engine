use std::{sync::Arc, time::Instant};

use mchess::{board::Board, moves::{magic::Magic, tables::AttackTables}};

fn perft(board: &mut Board, depth: u32) -> usize {
    if depth == 0 { return 1; }
    
    let moves = board.get_legal_moves();
    if depth == 1 { return moves.len(); }

    let mut nodes = 0;
    for m in moves {
        let state = board.make_move(&m);
        nodes += perft(board, depth - 1);
        board.unmake_move(&state);
    }

    nodes
}

fn split_perft(board: &mut Board, depth: u32) -> usize {
    if depth == 0 { return 1; }
    
    let moves = board.get_legal_moves();
    if depth == 1 { 
        for m in &moves {
            println!("{}: 1", m);
        }
        return moves.len();
    }

    let mut total_nodes = 0;
    for m in moves {
        let move_str = format!("{}", m);

        let state = board.make_move(&m);
        let nodes = perft(board, depth - 1);
        
        println!("{}: {}", move_str, nodes);
        
        total_nodes += nodes;
        board.unmake_move(&state);
    }

    total_nodes
}

fn main() {
    let magic = Arc::new(Magic::new());
    let attacks = Arc::new(AttackTables::new());
    let mut board = Board::startpos(magic, attacks);

    split_perft(&mut board, 2);

    for depth in 0..6 {
        let start = Instant::now();
    
        let res = perft(&mut board, depth);
    
        let duration = start.elapsed();
    
        println!("Depth: {}: {:?} = {}", depth, duration, res);
    }

    assert_eq!(perft(&mut board, 2), 400);
}