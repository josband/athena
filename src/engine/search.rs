use std::{
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};

use crate::chess::{
    Piece, Position, State,
    movegen::{Move, MoveList},
};

pub(crate) const PAWN_VALUE: Evaluation = Evaluation(100);
pub(crate) const KNIGHT_VALUE: Evaluation = Evaluation(300);
pub(crate) const BISHOP_VALUE: Evaluation = Evaluation(300);
pub(crate) const ROOK_VALUE: Evaluation = Evaluation(500);
pub(crate) const QUEEN_VALUE: Evaluation = Evaluation(1000);
pub(crate) const KING_VALUE: Evaluation = Evaluation(10000);

/// Core chess engine logic.
///
/// A chess engine consists of a [search algorithm](https://www.chessprogramming.org/Search)
/// and a [evaluation function](https://www.chessprogramming.org/Search)
pub trait Searcher {
    fn best_move(&mut self, pos: &mut Position, ply: u32, deadline: Instant) -> Option<Move>;

    /// Search Algorithm to find the best move
    ///
    /// # Returns
    ///
    /// The evaluation of the current position.
    ///
    /// Positive evaluations indicate an advantage for white while
    /// negative evaluations indicate an advantage for black.
    fn search(&mut self, pos: &mut Position, ply: u32, deadline: Instant) -> Evaluation;

    /// Evaluates the current position.
    fn eval(&self, pos: &Position) -> Evaluation;
}

pub struct NegamaxSearcher {
    stop: Arc<AtomicBool>,
    state_history: Vec<State>,
    time_checker: TimeChecker,
}

impl NegamaxSearcher {
    pub fn new(stop: Arc<AtomicBool>) -> Self {
        Self {
            stop,
            state_history: Vec::with_capacity(256),
            time_checker: TimeChecker::new(),
        }
    }
}

impl Searcher for NegamaxSearcher {
    fn best_move(&mut self, pos: &mut Position, ply: u32, deadline: Instant) -> Option<Move> {
        if ply == 0 {
            return None;
        }

        let mut best_eval = Evaluation::MIN;
        let mut best_move = None;
        for mv in MoveList::generate_for(pos, false) {
            if pos.make_move(mv, &mut self.state_history) {
                let eval = -self.search(pos, ply - 1, deadline);
                if best_eval < eval {
                    best_eval = eval;
                    best_move = Some(mv);
                }

                pos.unmake_move(mv, &mut self.state_history);
            }

            if self.time_checker.check(deadline) || self.stop.load(Ordering::SeqCst) {
                break;
            }
        }

        best_move
    }

    fn search(&mut self, pos: &mut Position, ply: u32, deadline: Instant) -> Evaluation {
        if ply == 0 {
            return self.eval(pos);
        }

        let mut best = Evaluation::MIN;
        for mv in MoveList::generate_for(pos, false) {
            if pos.make_move(mv, &mut self.state_history) {
                let eval = -self.search(pos, ply - 1, deadline);
                best = best.max(eval);
                pos.unmake_move(mv, &mut self.state_history);
            }

            if self.time_checker.check(deadline) || self.stop.load(Ordering::SeqCst) {
                break;
            }
        }

        best
    }

    fn eval(&self, pos: &Position) -> Evaluation {
        // For negamax to work, we must return the valuation with respect to the side to move
        let multiplier = if pos.side_to_move().is_white() { 1 } else { -1 };

        Evaluation::new(multiplier)
            * (KING_VALUE
                * (pos.piece_count(Piece::WHITE_KING) - pos.piece_count(Piece::BLACK_KING))
                + QUEEN_VALUE
                    * (pos.piece_count(Piece::WHITE_QUEEN) - pos.piece_count(Piece::BLACK_QUEEN))
                + ROOK_VALUE
                    * (pos.piece_count(Piece::WHITE_ROOK) - pos.piece_count(Piece::BLACK_ROOK))
                + BISHOP_VALUE
                    * (pos.piece_count(Piece::WHITE_BISHOP) - pos.piece_count(Piece::BLACK_BISHOP))
                + KNIGHT_VALUE
                    * (pos.piece_count(Piece::WHITE_KNIGHT) - pos.piece_count(Piece::BLACK_KNIGHT))
                + PAWN_VALUE
                    * (pos.piece_count(Piece::WHITE_PAWN) - pos.piece_count(Piece::BLACK_PAWN)))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TimeControls {
    /// Time remaining in milliseconds
    pub time: Option<Duration>,

    /// Increment in milliseconds per move
    pub increment: Option<Duration>,
}

struct TimeChecker {
    nodes: u64,
}

impl TimeChecker {
    pub fn new() -> Self {
        Self { nodes: 0 }
    }

    pub fn check(&mut self, deadline: Instant) -> bool {
        self.nodes += 1;
        if self.nodes.is_multiple_of(4095) && Instant::now() >= deadline {
            return true;
        }

        false
    }
}

/// Evaluation of a chess position.
///
/// The sign of the value can be with respect to the side to move
/// in a position, or it can be based on a fixed size where white
/// is positive and black is negative. '1' represents 1/100th of
/// a pawn (called a centipawn).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Evaluation(i32);

impl Evaluation {
    pub const MAX: Evaluation = Evaluation(i32::MAX);
    pub const MIN: Evaluation = Evaluation(i32::MIN);
    pub const EQUAL: Evaluation = Evaluation(0);

    pub fn new(val: i32) -> Self {
        Self(val)
    }
}

impl Neg for Evaluation {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl Add for Evaluation {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Evaluation {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub for Evaluation {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for Evaluation {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl Mul for Evaluation {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl MulAssign for Evaluation {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0
    }
}

impl Mul<i32> for Evaluation {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl MulAssign<i32> for Evaluation {
    fn mul_assign(&mut self, rhs: i32) {
        self.0 *= rhs
    }
}

impl Div for Evaluation {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl DivAssign for Evaluation {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0
    }
}

impl Div<i32> for Evaluation {
    type Output = Self;

    fn div(self, rhs: i32) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl DivAssign<i32> for Evaluation {
    fn div_assign(&mut self, rhs: i32) {
        self.0 /= rhs
    }
}
