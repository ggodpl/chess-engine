use crate::engine::{Engine, EngineStats};

pub fn display_results(engines: &Vec<Engine>) {
    let mut engines: Vec<(String, EngineStats)> = engines.iter()
        .map(|e| (e.name.to_owned(), e.stats))
        .collect();

    engines.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    for e in engines {
        println!("{} (score: {}) - {}", e.0, e.1.score, draw_bar(e.1.wins, e.1.draws, e.1.losses));
    }
}

fn draw_bar(wins: u32, draws: u32, losses: u32) -> String {
    const BAR_LENGTH: usize = 40;
    const GREEN: &str = "\x1B[92m█\x1B[39m";
    const RED: &str = "\x1B[91m█\x1B[39m";
    const GRAY: &str = "\x1B[90m█\x1B[39m";

    let games = wins + draws + losses;

    let s_wins = BAR_LENGTH * wins as usize / games as usize;
    let s_losses = BAR_LENGTH * losses as usize / games as usize;
    let s_draws = BAR_LENGTH - s_wins - s_losses;

    format!("{}{}{} ({}/{}/{})", GREEN.repeat(s_wins), GRAY.repeat(s_draws), RED.repeat(s_losses), wins, draws, losses)
}