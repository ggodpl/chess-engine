use std::sync::Arc;

use mchess::{board::Board, moves::{magic::Magic, tables::AttackTables}};

#[test]
fn test_hash_unmake() {
    let magic = Arc::new(Magic::new());
    let attacks = Arc::new(AttackTables::new());

    let mut board = Board::startpos(magic, attacks);

    let moves = board.get_legal_moves();

    for m in moves {
        let old_hash = board.hash;
        let state = board.make_move(m);

        board.unmake_move(&state);

        assert_eq!(old_hash, board.hash);
    }
}