use core::f64;

use crate::{board::Board, evaluation::evaluate};

use super::{Search, SearchResult};

impl Search {
    pub(crate) fn alphabeta(&mut self, board: &mut Board, depth: u8, _alpha: f64, _beta: f64, maximizer: bool) -> SearchResult {
        if self.is_stopping {
            return SearchResult {
                value: 0.0,
                moves: vec![]
            }
        }

        let mut alpha = _alpha;
        let mut beta = _beta;

        self.nodes += 1;

        if board.is_checkmate() || board.is_draw() || depth == 0 {
            return SearchResult {
                value: evaluate(board).to_value(),
                moves: vec![]
            }
        }

        if maximizer {
            let mut value = f64::NEG_INFINITY;
            let mut moves = vec![];

            let legal_moves = self.sort_moves(&board.get_legal_moves(), board);

            for (m, _) in legal_moves {
                let state = board.make_move(m);

                let result = self.alphabeta(board, depth - 1, alpha, beta, false);
                
                board.unmake_move(&state);

                if result.value > value {
                    value = result.value;

                    if !result.moves.is_empty() {
                        let mut new_moves = vec![m.clone()];
                        new_moves.extend(result.moves);
                        moves = new_moves;
                    } else {
                        moves = vec![m.clone()];
                    }
                }

                if value >= beta {
                    break;
                }

                alpha = alpha.max(value);
            }

            return SearchResult {
                value,
                moves
            }
        } else {
            let mut value = f64::INFINITY;
            let mut moves = vec![];

            let legal_moves = self.sort_moves(&board.get_legal_moves(), board);

            for (m, _) in legal_moves {
                let state = board.make_move(m);

                let result = self.alphabeta(board, depth - 1, alpha, beta, true);
                
                board.unmake_move(&state);

                if result.value < value {
                    value = result.value;
                    
                    if !result.moves.is_empty() {
                        let mut new_moves = vec![m.clone()];
                        new_moves.extend(result.moves);
                        moves = new_moves;
                    } else {
                        moves = vec![m.clone()];
                    }
                }

                if value <= alpha {
                    break;
                }

                beta = beta.min(value);
            }

            return SearchResult {
                value,
                moves
            }
        }
    }
}
