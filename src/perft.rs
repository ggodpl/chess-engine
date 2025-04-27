use crate::{board::Board, display::MoveDisplay};

pub fn perft(board: &mut Board, depth: u32) -> usize {
    if depth == 0 { return 1; }
    
    let moves = board.get_legal_moves();
    if depth == 1 { return moves.len(); }

    let mut nodes = 0;
    for m in moves {
        let state = board.make_move(m);
        nodes += perft(board, depth - 1);
        board.unmake_move(&state);
    }

    nodes
}

pub fn split_perft(board: &mut Board, depth: u32) -> usize {
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
        let move_str = format!("{}", MoveDisplay(m));

        let state = board.make_move(m);
        let nodes = perft(board, depth - 1);
        
        println!("{}: {}", move_str, nodes);
        
        total_nodes += nodes;
        board.unmake_move(&state);
    }

    total_nodes
}