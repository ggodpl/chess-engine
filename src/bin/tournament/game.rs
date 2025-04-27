use std::sync::Arc;

use mchess::{board::Board, display::MoveDisplay, moves::{magic::Magic, tables::AttackTables}};

use crate::engine::Engine;

const STARTPOS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub struct Game {
    pub games_per_match: usize,
    pub time_limit: u64,
    pub magic: Arc<Magic>,
    pub attacks: Arc<AttackTables>,
}

#[derive(Clone, Copy)]
pub enum GameResult {
    WhiteWin,
    BlackWin,
    Draw
}

impl Game {
    pub fn play_match(&self, engine1: &mut Engine, engine2: &mut Engine) -> Result<Vec<GameResult>, String> {
        let mut results = Vec::new();

        println!("Matching {} with {}", engine1.name, engine2.name);

        for game in 0..self.games_per_match {
            println!("{}-{}, game {}/{}", engine1.name, engine2.name, game, self.games_per_match);
            let result = if game % 2 == 0 {
                self.play_game(&mut *engine1, &mut *engine2)?
            } else {
                self.play_game(&mut *engine2, &mut *engine1)?
            };
        
            results.push(result);

            match result {
                GameResult::WhiteWin => {
                    if game % 2 == 0 {
                        engine1.stats.wins += 1;
                        engine1.stats.score += 1.0;
                        engine2.stats.losses += 1;
                    } else {
                        engine2.stats.wins += 1;
                        engine2.stats.score += 1.0;
                        engine1.stats.losses += 1;
                    }
                },
                GameResult::BlackWin => {
                    if game % 2 == 0 {
                        engine2.stats.wins += 1;
                        engine2.stats.score += 1.0;
                        engine1.stats.losses += 1;
                    } else {
                        engine1.stats.wins += 1;
                        engine1.stats.score += 1.0;
                        engine2.stats.losses += 1;
                    }
                },
                GameResult::Draw => {
                    engine1.stats.draws += 1;
                    engine2.stats.draws += 1;
                    engine1.stats.score += 0.5;
                    engine2.stats.score += 0.5;
                }
            }
        }

        Ok(results)
    }

    pub fn play_game(&self, white: &mut Engine, black: &mut Engine) -> Result<GameResult, String> {
        white.init()?;
        black.init()?;

        let mut moves = Vec::new();

        let mut board = Board::startpos(self.magic.clone(), self.attacks.clone());

        for _ in 0..400 {
            let white_move = white.get_best_move(STARTPOS, &moves, self.time_limit)?;

            for m in board.get_legal_moves() {
                if &format!("{}", MoveDisplay(m)) == &white_move {
                    board.make_move(m);
                }
            }

            moves.push(white_move);

            if board.is_checkmate() {
                return Ok(GameResult::WhiteWin);
            }
            if board.is_draw() {
                return Ok(GameResult::Draw);
            }

            let black_move = black.get_best_move(STARTPOS, &moves, self.time_limit)?;

            for m in board.get_legal_moves() {
                if &format!("{}", MoveDisplay(m)) == &black_move {
                    board.make_move(m);
                }
            }

            moves.push(black_move);

            if board.is_checkmate() {
                return Ok(GameResult::BlackWin);
            }
            if board.is_draw() {
                return Ok(GameResult::Draw);
            }
        }

        Ok(GameResult::Draw)
    }
}