use core::f64;

use crate::{board::Board, moves::Move};

pub mod minimax;

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub value: f64,
    pub moves: Vec<Move>
}

pub struct Search {
    pub is_stopping: bool,
    pub nodes: usize
}

impl Search {
    pub fn new() -> Self {
        Search { 
            is_stopping: false,
            nodes: 0
        }
    }

    pub fn search(&mut self, board: &mut Board, depth: u8) -> SearchResult {
        self.alphabeta(board, depth, f64::NEG_INFINITY, f64::INFINITY, true)
    }
}