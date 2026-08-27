use std::{
    io,
    str::FromStr as _,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Sender},
    },
    thread,
};

use uci_parser::{UciCommand, UciResponse, UciSearchOptions};

use crate::{
    chess::{
        Position,
        movegen::{self, Move, MoveList, init_movegen},
    },
    engine::{core::Athena, search::NegamaxSearcher},
};

#[derive(Debug, Default)]
pub struct UciEngine {
    worker: SearchHandle,
    position: Position,
}

impl UciEngine {
    pub fn new() -> Self {
        Self::default()
    }

    /// Starts the UCI engine loop
    ///
    /// Loops indefinitely, reading UCI commands from stdin and responding to them
    /// in stdout. The loop can be exited by sending the "quit" command.
    pub fn start(&mut self) {
        init_movegen();

        for line in io::stdin().lines() {
            if let Err(e) = line {
                println!("Failed to read from stdin: {}", e);
                break;
            }

            let Ok(cmd) = line.as_ref().unwrap().parse::<UciCommand>() else {
                if line
                    .as_ref()
                    .unwrap()
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
                    .trim()
                    .to_lowercase()
                    == "d"
                {
                    println!("{}", self.position);
                } else {
                    println!("Unknown command \"{}\"", line.unwrap());
                }

                continue;
            };

            match cmd {
                UciCommand::Uci => println!(
                    "{}\n{}\n\n{}",
                    UciResponse::Name("Athena"),
                    UciResponse::Author("josband"),
                    UciResponse::uciok()
                ),
                UciCommand::IsReady => println!("{}", UciResponse::readyok()),
                UciCommand::Position { fen, moves } => self.set_position(fen, moves),
                UciCommand::Go(options) => {
                    self.worker.start_searching(self.position.clone(), options);
                }
                UciCommand::Stop => self.worker.stop_searching(),
                // Pondering/Debug are not yet supported. They are optoinal features of UCI
                UciCommand::PonderHit | UciCommand::Debug(_) => (),
                // No state needs to be reset between games
                UciCommand::UciNewGame => (),
                // No options yet exist, do nothing
                UciCommand::SetOption { .. } => (),
                // Nothing to register - athena is open source
                UciCommand::Register { .. } => (),
                UciCommand::Quit => break,
            }
        }
    }

    fn set_position(&mut self, fen: Option<String>, moves: Vec<String>) {
        let mut pos = Position::default();
        if let Some(parsed_fen) = fen.as_ref().map(|s| Position::from_str(s)) {
            pos = match parsed_fen {
                Ok(p) => p,
                _ => return,
            };
        }

        let mut history = vec![];
        let mut success = true;
        for mv in moves {
            let next_move = Self::to_move(&mut pos, &mv);
            if let Some(next_move) = next_move {
                pos.make_move(next_move, &mut history);
            } else {
                success = false;
                break;
            }
        }

        if success {
            self.position = pos;
        }
    }

    fn to_move(pos: &mut Position, mv: &str) -> Option<Move> {
        let mut legal_moves = MoveList::new();
        movegen::generate_legal_moves(pos, &mut legal_moves);
        legal_moves.into_iter().find(|m| m.to_uci_string() == mv)
    }
}

#[derive(Debug)]
pub struct SearchHandle {
    tx: Sender<(Position, UciSearchOptions)>,
    stop: Arc<AtomicBool>,
}

impl SearchHandle {
    pub fn new() -> Self {
        let stop = Arc::new(AtomicBool::new(false));
        let engine_stop = stop.clone();
        let (tx, rx) = mpsc::channel::<(Position, UciSearchOptions)>();
        let searcher = NegamaxSearcher::new(stop.clone());
        let mut engine = Athena::new(engine_stop, searcher);
        thread::spawn(move || {
            while let Ok((pos, limits)) = rx.recv() {
                if let Some(mv) = engine.go(pos, limits) {
                    println!(
                        "{}",
                        UciResponse::BestMove {
                            bestmove: Some(mv.to_uci_string()),
                            ponder: None
                        }
                    );
                };
            }
        });

        Self { tx, stop }
    }

    pub fn start_searching(&self, pos: Position, limits: UciSearchOptions) {
        let _ = self.tx.send((pos, limits));
    }

    pub fn stop_searching(&self) {
        self.stop.store(true, Ordering::Relaxed);
    }
}

impl Default for SearchHandle {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for SearchHandle {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
    }
}
