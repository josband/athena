use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};

use uci_parser::{UciInfo, UciSearchOptions};

use crate::{
    chess::{Position, movegen::Move},
    engine::search::Searcher,
};

pub(crate) const MAX_DEPTH: u32 = 64;

/// Athena Chess Engine
///
/// This is the core of the Athena engine.
#[derive(Debug, Clone)]
pub struct Athena<T: Searcher> {
    stop: Arc<AtomicBool>,
    searcher: T,
}

impl<T: Searcher> Athena<T> {
    /// Initializes Athena from a new position
    pub fn new(stop: Arc<AtomicBool>, searcher: T) -> Self {
        Self { stop, searcher }
    }

    /// Starts the search for a best move
    pub fn go(&mut self, pos: Position, limits: UciSearchOptions) -> Option<Move> {
        self.stop.store(false, Ordering::SeqCst);

        let depth = self.max_depth(&limits);
        let deadline = self.deadline(&pos, &limits);

        let best = self.iterative_deepening(pos, depth, deadline);

        self.stop.store(false, Ordering::SeqCst);

        best
    }

    /// Iterative deepening search for best move.
    ///
    /// Iterative deepening is a search strategy that repeatedly applies a depth-limited search
    /// with increasing depth limits until a max depth is hit or a time limit is reached. This
    /// allows an engine to incrementally improve its best move as the depth increases. Information
    /// from shallower searches can be used to improve the efficiency of deeper searches.
    fn iterative_deepening(
        &mut self,
        mut pos: Position,
        depth: u32,
        deadline: Instant,
    ) -> Option<Move> {
        if depth == 0 {
            return None;
        }

        let mut best = None;
        for depth in 1..depth + 1 {
            best = self.searcher.best_move(&mut pos, depth, deadline);
            let info = UciInfo::new().depth(depth);
            println!("{}", info);
            // if Instant::now() >= deadline || self.stop.load(Ordering::SeqCst) {
            //     break;
            // }
        }

        best
    }

    fn max_depth(&self, limits: &UciSearchOptions) -> u32 {
        limits.depth.filter(|d| *d != 0).unwrap_or(MAX_DEPTH)
    }

    fn deadline(&self, pos: &Position, limits: &UciSearchOptions) -> Instant {
        let us = pos.side_to_move();
        let time = if us.is_white() {
            limits.wtime
        } else {
            limits.btime
        };

        let inc = if us.is_white() {
            limits.winc
        } else {
            limits.binc
        };

        time.map(|t| Instant::now() + (t / 30 + inc.unwrap_or(Duration::from_millis(0)) / 2))
            .unwrap_or_else(|| Instant::now() + Duration::from_hours(10))
    }
}
