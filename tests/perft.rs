use std::sync::Arc;

use mchess::{board::Board, moves::{magic::Magic, tables::AttackTables}};

fn test_fen(expected: &[usize], fen: &str) {
    let magic = Arc::new(Magic::new());
    let attacks = Arc::new(AttackTables::new());

    let mut board = Board::from_fen(fen, magic.clone(), attacks.clone());

    for depth in 0..expected.len() {
        let start = std::time::Instant::now();
        let result = mchess::perft::perft(&mut board, depth as u32);
        let duration = start.elapsed();
        
        assert_eq!(result, expected[depth], "Perft failed at depth {}", depth);
        println!("Perft depth {} = {} nodes in {:?}", depth, result, duration);

        board = Board::from_fen(fen, magic.clone(), attacks.clone());
    }
}

#[test]
fn perft() {
    let expected = [
        1,         // depth 0
        20,        // depth 1
        400,       // depth 2
        8902,      // depth 3
        197281,    // depth 4
        4865609,   // depth 5
        119060324, // depth 6
        // 3195901860 // depth 7
    ];

    test_fen(&expected, "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
}

#[test]
fn kiwipete() {
    let expected = [
        1,         // depth 0
        48,        // depth 1
        2039,      // depth 2
        97862,     // depth 3
        4085603,   // depth 4
        193690690, // depth 5
    ];
    
    test_fen(&expected, "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1");
}

#[test]
fn position3() {
    let expected = [
        1,         // depth 0
        14,        // depth 1
        191,       // depth 2
        2812,      // depth 3
        43238,     // depth 4
        674624,    // depth 5
        11030083,  // depth 6
        178633661, // depth 7
        3009794393 // depth 8
    ];

    test_fen(&expected, "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1 ");
}