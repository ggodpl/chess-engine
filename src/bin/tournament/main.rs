use std::env;

use tournament::Tournament;

mod engine;
mod game;
mod tournament;
mod display;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: tournament <path>");
        return Err("Usage: tournament <path>".to_string());
    }

    Tournament::load(&args[1])
}