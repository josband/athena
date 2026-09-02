use std::{
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};

use uci_parser::{UciInfo, UciSearchOptions};

use crate::chess::{
    Piece, Position, State,
    movegen::{Move, MoveList},
};

pub(crate) const MAX_DEPTH: u32 = 64;
pub(crate) const PAWN_VALUE: Evaluation = Evaluation(100);
pub(crate) const KNIGHT_VALUE: Evaluation = Evaluation(300);
pub(crate) const BISHOP_VALUE: Evaluation = Evaluation(300);
pub(crate) const ROOK_VALUE: Evaluation = Evaluation(500);
pub(crate) const QUEEN_VALUE: Evaluation = Evaluation(1000);
pub(crate) const KING_VALUE: Evaluation = Evaluation(10000);

type ThrottleArgs<'a> = (Instant, &'a Arc<AtomicBool>);
type ThrottledStop = Throttle<bool, fn(ThrottleArgs) -> bool>;

/// Athena Chess Engine
///
/// This is the core of the Athena engine.
#[derive(Debug)]
pub struct Athena {
    stop: Arc<AtomicBool>,
    stopping: bool,
    stop_check: ThrottledStop,
    state_history: Vec<State>,
}

impl Athena {
    /// Initializes Athena from a new position
    pub fn new(stop: Arc<AtomicBool>) -> Self {
        Self {
            stop,
            stop_check: Throttle::new(
                |(deadline, stop)| Instant::now() > deadline || stop.load(Ordering::SeqCst),
                false,
            ),
            stopping: false,
            state_history: Vec::with_capacity(256),
        }
    }

    /// Starts the search for a best move
    pub fn go(&mut self, pos: Position, limits: UciSearchOptions) -> Option<Move> {
        self.stopping = false;
        self.stop.store(false, Ordering::SeqCst);

        let depth = self.max_depth(&limits);
        let deadline = self.deadline(&pos, &limits);

        let best = self.iterative_deepening(pos, depth, deadline);

        self.stop.store(false, Ordering::SeqCst);
        self.stopping = false;

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
            best = self.negamax_root(&mut pos, depth, deadline);
            let info = UciInfo::new().depth(depth);
            println!("info{}", info);

            if self.should_terminate_search(deadline) {
                break;
            }
        }

        self.stop_check.reset_limit();

        best
    }

    fn should_terminate_search(&mut self, deadline: Instant) -> bool {
        let res = self.stopping || self.stop_check.tick((deadline, &self.stop));
        if res {
            self.stopping = true;
        }

        res
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

    fn negamax_root(&mut self, pos: &mut Position, ply: u32, deadline: Instant) -> Option<Move> {
        if ply == 0 {
            return None;
        }

        let mut best_eval = Evaluation::MIN;
        let mut best_move = None;
        for mv in MoveList::generate_for(pos, false) {
            if pos.make_move(mv, &mut self.state_history) {
                let eval = -self.negamax(pos, ply - 1, deadline);
                if best_eval < eval {
                    best_eval = eval;
                    best_move = Some(mv);
                }

                pos.unmake_move(mv, &mut self.state_history);
            }

            if self.should_terminate_search(deadline) {
                break;
            }
        }

        best_move
    }

    fn negamax(&mut self, pos: &mut Position, ply: u32, deadline: Instant) -> Evaluation {
        if ply == 0 {
            return self.eval(pos);
        }

        let mut best = Evaluation::MIN;
        for mv in MoveList::generate_for(pos, false) {
            if pos.make_move(mv, &mut self.state_history) {
                let eval = -self.negamax(pos, ply - 1, deadline);
                best = best.max(eval);
                pos.unmake_move(mv, &mut self.state_history);
            }

            if self.should_terminate_search(deadline) {
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

/// Evaluation of a chess position.
///
/// The sign of the value can be with respect to the side to move
/// in a position, or it can be based on a fixed size where white
/// is positive and black is negative. '1' represents 1/100th of
/// a pawn (called a centipawn).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Evaluation(i32);

impl Evaluation {
    pub const MIN: Evaluation = Evaluation(-1000000);
    pub const MAX: Evaluation = Evaluation(1000000);
    pub const DRAW: Evaluation = Evaluation(0);

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Throttle<T, F> {
    ticks: u64,
    func: F,
    default: T,
}

impl<T, F> Throttle<T, F> {
    pub fn reset_limit(&mut self) {
        self.ticks = 0;
    }
}

impl<T, F> Throttle<T, F>
where
    T: Clone,
{
    pub fn new<U>(func: F, default: T) -> Self
    where
        F: Fn(U) -> T,
    {
        Self {
            ticks: 0,
            func,
            default,
        }
    }

    pub fn tick<U>(&mut self, arg: U) -> T
    where
        F: Fn(U) -> T,
    {
        self.ticks += 1;
        if self.ticks.is_multiple_of(4095) {
            return (self.func)(arg);
        }

        self.default.clone()
    }
}
