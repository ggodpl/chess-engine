use core::f64;

use crate::{board::Board, evaluation::evaluate, moves::{helper::{get_color, get_piece_type, get_to, is_capture}, Move}, piece::Piece};

use super::{Node, NodeType, Search, SearchResult};

impl Search {
    pub(super) fn alphabeta(&mut self, board: &mut Board, depth: u8, mut alpha: f64, mut beta: f64, maximizer: bool) -> SearchResult {
        if self.is_stopping {
            return SearchResult {
                value: 0.0,
                moves: vec![]
            }
        }

        self.nodes += 1;

        if board.is_checkmate() || board.is_draw() || depth == 0 {
            return SearchResult {
                value: evaluate(board).to_value(),
                moves: vec![]
            }
        }

        let hash = board.hash;

        if let Some(entry) = self.tt.get(&hash) {
            if entry.generation == self.current_generation && entry.depth >= depth {
                self.tt_hits += 1;
                match entry.node_type {
                    NodeType::PV => {
                        return SearchResult {
                            value: entry.value,
                            moves: entry.best_move.map_or(vec![], |m| vec![m])
                        };
                    },
                    NodeType::Cut if entry.value >= beta => {
                        return SearchResult {
                            value: entry.value,
                            moves: entry.best_move.map_or(vec![], |m| vec![m])
                        };
                    },
                    NodeType::All if entry.value <= alpha => {
                        return SearchResult {
                            value: entry.value,
                            moves: entry.best_move.map_or(vec![], |m| vec![m])
                        };
                    },
                    _ => {}
                }
            }
        }

        if maximizer {
            let mut value = f64::NEG_INFINITY;
            let mut moves = vec![];

            let mut node_type = NodeType::All;

            let legal_moves = self.sort_moves(&board.get_legal_moves(), board, depth);

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

                if value > alpha {
                    alpha = value;
                    node_type = NodeType::PV;
                }

                if value >= beta {
                    node_type = NodeType::Cut;
                    self.store_killer_move(&m, depth);
                    self.store_history(&m, depth);
                    break;
                }

            }

            self.store_tt(hash, Node {
                depth,
                node_type,
                value,
                best_move: moves.first().copied(),
                generation: self.current_generation,
            });

            return SearchResult {
                value,
                moves
            }
        } else {
            let mut value = f64::INFINITY;
            let mut moves = vec![];

            let mut node_type = NodeType::All;

            let legal_moves = self.sort_moves(&board.get_legal_moves(), board, depth);

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

                if value < beta {
                    beta = value;
                    node_type = NodeType::PV;
                }

                if value <= alpha {
                    node_type = NodeType::Cut;
                    self.store_killer_move(&m, depth);
                    self.store_history(&m, depth);
                    break;
                }
            }

            self.store_tt(hash, Node {
                depth,
                node_type,
                value,
                best_move: moves.first().copied(),
                generation: self.current_generation,
            });
            
            return SearchResult {
                value,
                moves
            }
        }
    }

    fn store_tt(&mut self, hash: i64, node: Node) {
        if let Some(existing) = self.tt.get(&hash) {
            if existing.generation == node.generation && existing.depth < node.depth {
                return;
            }
        }

        self.tt.insert(hash, node);
    }

    fn store_killer_move(&mut self, m: &Move, depth: u8) {
        if !is_capture(*m) {
            if Some(m) != self.killer_moves[depth as usize][0].as_ref() {
                self.killer_moves[depth as usize][1] = self.killer_moves[depth as usize][0];
                self.killer_moves[depth as usize][0] = Some(*m);
            }
        }
    }

    fn store_history(&mut self, m: &Move, depth: u8) {
        let m = *m;

        let piece_index = Piece::index_from(get_piece_type(m), get_color(m));
        let to = get_to(m).trailing_zeros() as usize;
        
        self.history[piece_index][to] += (depth as i32) * (depth as i32);

        if self.history[piece_index][to] > 10000 {
            for p in 0..12 {
                for s in 0..64 {
                    self.history[p][s] /= 2;
                }
            }
        }
    }
}
