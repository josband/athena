//! Chess playing decision-making & reasoning
//!
//! This module is responsible for playing chess and contains
//! all of the pieces to not only play a game correctly, but
//! efficiently.

pub mod core;

use std::time::{Duration, Instant};

use crate::{
    chess::{
        Color, PerColor, Piece, Position, State,
        movegen::{Move, MoveList},
    },
    engine::core::{
        BISHOP_VALUE, Evaluation, KING_VALUE, KNIGHT_VALUE, PAWN_VALUE, QUEEN_VALUE, ROOK_VALUE,
    },
};

const MAX_DEPTH: u32 = 64;

/// Core chess engine logic.
///
/// A chess engine consists of a [search algorithm](https://www.chessprogramming.org/Search)
/// and a [evaluation function](https://www.chessprogramming.org/Search)
pub trait Engine {
    /// Search Algorithm to find the best move
    ///
    /// # Arguments
    ///
    /// * `ply` - The number of half-moves to search
    /// * `deadline` - The time at which the search should be aborted
    ///
    /// # Returns
    ///
    /// The evaluation of the current position.
    ///
    /// Positive evaluations indicate an advantage for white while
    /// noegative evaluations indicate an advantage for black.
    fn search(&mut self, ply: u32, deadline: Instant) -> Evaluation;

    /// Evaluates the current position.
    fn eval(&self) -> Evaluation;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct TimeControls {
    time: Duration,
    increment: Duration,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Athena {
    pos: Position,
    state_history: Vec<State>,
    time_controls: PerColor<TimeControls>,
}

impl Athena {
    pub fn new(pos: Position) -> Self {
        Self {
            pos,
            state_history: vec![],
            time_controls: PerColor::new(TimeControls {
                time: Duration::MAX,
                increment: Duration::ZERO,
            }),
        }
    }

    pub fn set_white_time(&mut self, time: Duration) {
        self.time_controls[Color::White].time = time;
    }

    pub fn set_white_increment(&mut self, inc: Duration) {
        self.time_controls[Color::White].increment = inc;
    }

    pub fn set_black_time(&mut self, time: Duration) {
        self.time_controls[Color::Black].time = time;
    }

    pub fn set_black_increment(&mut self, inc: Duration) {
        self.time_controls[Color::Black].increment = inc;
    }

    pub fn best_move(&mut self) -> Option<Move> {
        let mut best = None;
        let us = self.pos.side_to_move();
        let max_search = self.time_controls[us].time / 20 + self.time_controls[us].increment / 2;
        let deadline = Instant::now() + max_search;
        for depth in 1..MAX_DEPTH {
            best = self.negamax_root(depth, deadline);

            if Instant::now() >= deadline {
                break;
            }
        }

        best
    }

    fn negamax_root(&mut self, ply: u32, deadline: Instant) -> Option<Move> {
        if ply == 0 {
            return None;
        }

        let mut best_eval = Evaluation::MIN;
        let mut best_move = None;
        for mv in MoveList::generate_for(&self.pos, false) {
            if self.pos.make_move(mv, &mut self.state_history) {
                let eval = -self.search(ply - 1, deadline);
                if best_eval < eval {
                    best_eval = best_eval.max(eval);
                    best_move = Some(mv);
                }

                self.pos.unmake_move(mv, &mut self.state_history);
            }

            if Instant::now() >= deadline {
                break;
            }
        }

        best_move
    }
}

impl Engine for Athena {
    fn search(&mut self, ply: u32, deadline: Instant) -> Evaluation {
        if ply == 0 {
            return self.eval();
        }

        let mut best = Evaluation::MIN;
        for mv in MoveList::generate_for(&self.pos, false) {
            if self.pos.make_move(mv, &mut self.state_history) {
                let eval = -self.search(ply - 1, deadline);
                best = best.max(eval);
                self.pos.unmake_move(mv, &mut self.state_history);
            }

            if Instant::now() >= deadline {
                break;
            }
        }

        best
    }

    fn eval(&self) -> Evaluation {
        // For negamax to work, we must return the valuation with respect to the side to move
        let multiplier = if self.pos.side_to_move().is_white() {
            1
        } else {
            -1
        };

        Evaluation::new(multiplier)
            * (KING_VALUE
                * (self.pos.piece_count(Piece::WHITE_KING)
                    - self.pos.piece_count(Piece::BLACK_KING))
                + QUEEN_VALUE
                    * (self.pos.piece_count(Piece::WHITE_QUEEN)
                        - self.pos.piece_count(Piece::BLACK_QUEEN))
                + ROOK_VALUE
                    * (self.pos.piece_count(Piece::WHITE_ROOK)
                        - self.pos.piece_count(Piece::BLACK_ROOK))
                + BISHOP_VALUE
                    * (self.pos.piece_count(Piece::WHITE_BISHOP)
                        - self.pos.piece_count(Piece::BLACK_BISHOP))
                + KNIGHT_VALUE
                    * (self.pos.piece_count(Piece::WHITE_KNIGHT)
                        - self.pos.piece_count(Piece::BLACK_KNIGHT))
                + PAWN_VALUE
                    * (self.pos.piece_count(Piece::WHITE_PAWN)
                        - self.pos.piece_count(Piece::BLACK_PAWN)))
    }
}
