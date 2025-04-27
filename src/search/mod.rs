use core::f64;
use std::time::Instant;

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
        self.is_stopping = false;
        self.alphabeta(board, depth, f64::NEG_INFINITY, f64::INFINITY, true)
    }

    pub fn iterative_deepening(&mut self, board: &mut Board, max_depth: u8, time_limit: u64) -> SearchResult {
        self.is_stopping = false;
        let start = Instant::now();

        let mut best_result = SearchResult { value: 0.0, moves: vec![] };
        
        for depth in 1..=max_depth {
            let result = self.search(board, depth);
            
            if self.is_stopping {
                break;
            }

            best_result = result;

            let elapsed = start.elapsed().as_millis() as u64;
            if elapsed > (time_limit * 3) / 4 {
                break;
            }

            println!("depth {depth}: {}", best_result);
        }

        best_result
    }

    pub fn search_infinite(&mut self, board: &mut Board) -> SearchResult {
        let mut depth = 1;
        let mut best_result = SearchResult { value: 0.0, moves: vec![] };

        loop {
            let result = self.search(board, depth);
            
            if self.is_stopping {
                break;
            }

            best_result = result;
            depth += 1;

            println!("depth {depth}: {}", best_result)
        }

        best_result
    }

    pub fn stop(&mut self) {
        self.is_stopping = true;
    }
}