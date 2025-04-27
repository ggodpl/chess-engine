use std::{cmp::Ordering, io::{BufRead, BufReader, Write}, process::{Child, Command, Stdio}};

pub struct Engine {
    pub name: String,
    process: Child,
    pub stats: EngineStats
}

#[derive(Clone, Copy)]
pub struct EngineStats {
    pub score: f32,
    pub wins: u32,
    pub losses: u32,
    pub draws: u32
}

impl PartialEq for EngineStats {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score 
        && self.wins == other.wins 
        && self.losses == other.losses 
        && self.draws == other.draws
    }
}

impl PartialOrd for EngineStats {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.score.partial_cmp(&other.score) {
            Some(Ordering::Equal) => Some(self.wins.cmp(&other.wins)),
            ordering => ordering
        }
    }
}

impl EngineStats {
    pub fn new() -> Self {
        EngineStats {
            score: 0.0,
            wins: 0,
            losses: 0,
            draws: 0
        }
    }
}

impl Engine {
    pub fn new(path: &str, name: &str) -> Result<Self, String> {
        let process = Command::new(path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to launch engine: {}", e))?;

        Ok(Engine {
            name: name.to_owned(),
            process,
            stats: EngineStats::new()
        })
    }

    pub fn send(&mut self, command: &str) -> Result<(), String> {
        println!("sending uci command to {}: {}", self.name, command);
        if let Some(stdin) = &mut self.process.stdin {
            writeln!(stdin, "{}", command)
                .map_err(|e| format!("Failed to send command: {}", e))?;
            stdin.flush()
                .map_err(|e| format!("Failed to flush stdin: {}", e))?;
            Ok(())
        } else {
            Err("Engine stdin not available".to_string())
        }
    }

    pub fn get(&mut self, command: Option<&str>) -> Result<String, String> {
        let stdout = self.process.stdout.as_mut()
            .ok_or_else(|| "Engine stdout not available".to_string())?;

        let mut reader = BufReader::new(stdout);
        let mut response = String::new();
        let mut line = String::new();

        loop {
            line.clear();
            match reader.read_line(&mut line) {
                Ok(0) => break,
                Ok(_) => {
                    response.push_str(&line);
                    
                    if let Some(command) = command {
                        if line.contains(command) {
                            break;
                        }
                    } else if !line.trim().is_empty() {
                        break;
                    }
                }
                Err(e) => return Err(format!("Error reading engine output: {}", e))
            }
        }

        if response.trim().is_empty() {
            return Err("Engine provided no response".to_string());
        }

        Ok(response)
    }

    pub fn init(&mut self) -> Result<(), String> {
        self.send("ucinewgame")?;
        self.send("uci")?;
        let response = self.get(Some("uciok"))?;

        if !response.contains("uciok") {
            return Err("Engine did not respond to uci".to_string());
        }

        self.send("isready")?;
        let response = self.get(Some("readyok"))?;
        if !response.contains("readyok") {
            return Err("Engine not ready".to_string());
        }

        Ok(())
    }

    pub fn get_best_move(&mut self, position: &str, moves: &[String], time_ms: u64) -> Result<String, String> {
        self.send(&format!("position fen {} moves {}", position, moves.join(" ")))?;

        self.send(&format!("go movetime {}", time_ms))?;

        let response = self.get(Some("bestmove"))?;

        if let Some(response) = response.lines().find(|line| line.starts_with("bestmove")) {
            let parts: Vec<&str> = response.split_whitespace().collect();
            if parts.len() >= 2 {
                return Ok(parts[1].to_string());
            }
        }

        Err("Failed to get the best move".to_string())
    }
}