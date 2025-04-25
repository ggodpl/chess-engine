use std::{sync::Arc, time::Instant};

use mchess::{board::Board, moves::{magic::Magic, tables::AttackTables}, perft::{perft, split_perft}};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let magic = Arc::new(Magic::new());
    let attacks = Arc::new(AttackTables::new());

    let default_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let default_depth = 5;

    if args.len() > 1 && (args[1] == "--help" || args[1] == "-h") {
        println!("Usage: perft [FEN] [depth] [-s]");
        println!("  FEN          - Chess position in FEN notation (default: startpos)");
        println!("  depth        - Search depth (default: 5)");
        println!("  -s (--split) - Split perft");
        println!("  -S (--short) - Don't display diagnostic information");
        return;
    }

    let split = args.iter().any(|arg| arg == "-s" || arg == "--split");
    let short = args.iter().any(|arg| arg == "-S" || arg == "--short");

    let fen = args.iter().skip(1)
        .find(|arg| !arg.starts_with('-'))
        .map(|s| s.as_str())
        .unwrap_or(default_fen);

    let depth = args.iter().skip(1)
        .filter(|arg| !arg.starts_with('-'))
        .nth(1)
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(default_depth);

    let mut board = Board::from_fen(fen, magic.clone(), attacks.clone());

    if !short {
        println!("Position:");
        println!("{}", board);
    }

    let start = Instant::now();
    let nodes = if split {
        split_perft(&mut  board, depth)
    } else {
        perft(&mut board, depth)
    };
    let duration = start.elapsed();

    if !short {
        println!("Time: {:?}", duration);
    }

    println!("{}", nodes);
}