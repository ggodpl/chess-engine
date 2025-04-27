use std::{io::{self, Write}, sync::Arc};

use crate::{board::Board, display::MoveDisplay, moves::{magic::Magic, tables::AttackTables}, piece::PieceColor, search::Search};

pub struct Uci {
    pub magic: Arc<Magic>,
    pub attacks: Arc<AttackTables>,
    pub board: Board,
    pub search: Search,
}

impl Uci {
    pub fn new() -> Self {
        let magic = Arc::new(Magic::new());
        let attacks = Arc::new(AttackTables::new());

        let board = Board::startpos(magic.clone(), attacks.clone());

        Uci {
            magic,
            attacks,
            board,
            search: Search::new(),
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        let stdin = io::stdin();
        let mut stdout = io::stdout();
        
        println!("mchess");

        let mut input = String::new();

        loop {
            input.clear();
            stdin.read_line(&mut input)?;
            let command = input.trim();

            if command == "quit" {
                break;
            } else {
                self.command(command)?;
            }

            stdout.flush().unwrap();
        }

        Ok(())
    }

    pub fn identify(&self) {
        println!("id name mchess");
        println!("id author ggod");
        println!("uciok");
    }

    pub fn command(&mut self, command: &str) -> io::Result<()> {
        let tokens: Vec<&str> = command.split_whitespace().collect();
        if tokens.is_empty() { return Ok(()); }

        match tokens[0] {
            "uci" => self.identify(),
            "isready" => println!("readyok"),
            "ucinewgame" => {
                self.board = Board::startpos(self.magic.clone(), self.attacks.clone());
            },
            "position" => self.handle_position(&tokens[1..]),
            "go" => self.handle_go(&tokens[1..]),
            "stop" => self.search.stop(),
            "quit" => {},
            a => println!("info string unknown option {}", a)
        }
        Ok(())
    }

    pub fn handle_position(&mut self, args: &[&str]) {
        if args.is_empty() { return; }

        let mut index = 0;

        if args[index] == "startpos" {
            self.board = Board::startpos(self.magic.clone(), self.attacks.clone());
            index += 1;
        } else if args[index] == "fen" {
            index += 1;
            let mut fen = String::new();
            while index < args.len() && args[index] != "moves" {
                if !fen.is_empty() { fen.push(' '); }
                fen.push_str(args[index]);

                index += 1;
            }

            self.board = Board::from_fen(&fen, self.magic.clone(), self.attacks.clone());
        }

        if index < args.len() && args[index] == "moves" {
            index += 1;
            while index < args.len() {
                self.make_move(args[index]);
                index += 1;
            }
        }
    }

    pub fn handle_go(&mut self, args: &[&str]) {
        let mut time_limit = None;
        let mut depth = None;

        let mut wtime = None;
        let mut btime = None;
        let mut winc = None;
        let mut binc = None;
        let mut movestogo = None;
        let mut movetime = None;

        let mut infinite = false;

        let mut i = 0;
        while i < args.len() {
            match args[i] {
                "movetime" => if let Ok(mt) = args[i + 1].parse::<u64>() {
                    movetime = Some(mt);
                },
                "depth" => if let Ok(d) = args[i + 1].parse::<u8>() {
                    depth = Some(d);
                },
                "wtime" => if let Ok(t) = args[i + 1].parse::<u64>() {
                    wtime = Some(t);
                },
                "btime" => if let Ok(t) = args[i + 1].parse::<u64>() {
                    btime = Some(t);
                },
                "winc" => if let Ok(inc) = args[i + 1].parse::<u64>() {
                    winc = Some(inc);
                },
                "binc" => if let Ok(inc) = args[i + 1].parse::<u64>() {
                    binc = Some(inc);
                },
                "movestogo" => if let Ok(mtg) = args[i + 1].parse::<u32>() {
                    movestogo = Some(mtg);
                },
                "infinite" => infinite = true,
                _ => {}
            }
            i += 1;
        }

        if let Some(mt) = movetime {
            time_limit = Some(mt);
        } else if wtime.is_some() || btime.is_some() {
            let is_white = self.board.turn == PieceColor::White;
            let time = if is_white { wtime } else { btime };
            let inc = if is_white { winc } else { binc };

            if let Some(remaining) = time {
                let moves_left = movestogo.unwrap_or(30);
                let increment = inc.unwrap_or(0);

                let base_time = remaining / moves_left as u64;
                let allocated = base_time + increment / 2;

                time_limit = Some(std::cmp::min(allocated, remaining / 5));
            }
        }

        let result = if let Some(time) = time_limit {
            self.search.iterative_deepening(&mut self.board, depth.unwrap_or(u8::MAX), time)
        } else if infinite {
            self.search.search_infinite(&mut self.board)
        } else {
            self.search.search(&mut self.board, depth.unwrap_or(5))
        };

        let best_move = result.moves.first();

        if let Some(m) = best_move.as_ref() {
            println!("bestmove {}", MoveDisplay(**m));
        } else {
            println!("bestmove 0000");
        }
    }

    pub fn make_move(&mut self, move_str: &str) {
        let m = self.board.parse_uci_string(move_str);

        if let Some(m) = m {
            self.board.make_move(m);
        }
    }
}