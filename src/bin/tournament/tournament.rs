use std::{fs, sync::Arc};

use mchess::moves::{magic::Magic, tables::AttackTables};

use crate::{display::display_results, engine::Engine, game::Game};

pub struct EngineInfo {
    pub name: String,
    pub path: String,
}

pub struct Tournament {
    pub engines: Vec<Engine>,
    pub handler: Game,
}

impl Tournament {
    pub fn load(path: &str) -> Result<(), String> {
        let config = fs::read_to_string(path)
            .expect("Failed to read config");

        let mut engines = Vec::new();
        let mut games_per_match = 2;
        let mut time_limit = 4000;

        for line in config.lines() {
            if line.trim().starts_with("engine:") {
                let parts: Vec<&str> = line["engine:".len()..].split_whitespace().collect();

                if parts.len() == 2 {
                    let path = parts[0].to_string();
                    let name = parts[1].trim_matches('"').to_string();
                    engines.push(EngineInfo {
                        path,
                        name
                    });
                }
            } else if line.trim().starts_with("games:") {
                if let Ok(value) = line["games:".len()..].trim().parse::<u32>() {
                    games_per_match = value as usize;
                }
            } else if line.trim().starts_with("time:") {
                if let Ok(value) = line["time:".len()..].trim().parse::<u32>() {
                    time_limit = value as u64;
                }
            }
        }

        if engines.len() < 2 {
            return Err("Too little engines!".to_string());
        }

        println!("Starting tournament with {} engines", engines.len());
        println!("Games per match: {}", games_per_match);
        println!("Time control: {}", time_limit);

        let magic = Arc::new(Magic::new());
        let attacks = Arc::new(AttackTables::new());

        let engines = engines.iter()
            .map(|info| Engine::new(&info.path, &info.name))
            .collect::<Result<Vec<Engine>, _>>()?;

        let mut tournament = Tournament {
            engines,
            handler: Game {
                games_per_match,
                time_limit,
                magic,
                attacks,
            }
        };

        tournament.run()
    }

    pub fn run(&mut self) -> Result<(), String> {
        for i in 0..self.engines.len() {
            for j in (i + 1)..self.engines.len() {
                let (left, right) = self.engines.split_at_mut(j);
                self.handler.play_match(&mut left[i], &mut right[0])?;
            }
        }

        display_results(&self.engines);

        Ok(())
    }
}