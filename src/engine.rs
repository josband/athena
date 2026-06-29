//! Chess playing decision-making & reasoning
//!
//! This module is responsible for playing chess and contains
//! all of the pieces to not only play a game correctly, but
//! efficiently.

pub mod core;

use crate::{
    chess::{Piece, Position, State, movegen::MoveList},
    engine::core::{
        BISHOP_VALUE, Evaluation, KING_VALUE, KNIGHT_VALUE, PAWN_VALUE, QUEEN_VALUE, ROOK_VALUE,
    },
};

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
    ///
    /// # Returns
    ///
    /// The evaluation of the current position.
    ///
    /// Positive evaluations indicate an advantage for white while
    /// noegative evaluations indicate an advantage for black.
    fn search(&mut self, ply: u8) -> Evaluation;

    /// Evaluates the current position.
    fn eval(&self) -> Evaluation;
}

#[derive(Debug, PartialEq, Eq)]
pub struct Athena {
    pos: Position,
    state_history: Vec<State>,
}

impl Athena {
    pub fn new(pos: Position) -> Self {
        Self {
            pos,
            state_history: vec![],
        }
    }
}

impl Engine for Athena {
    fn search(&mut self, ply: u8) -> Evaluation {
        if ply == 0 {
            return self.eval();
        }

        let mut best = Evaluation::MIN;
        for mv in MoveList::generate_for(&self.pos, false) {
            if self.pos.make_move(mv, &mut self.state_history) {
                let eval = -self.search(ply - 1);
                best = best.max(eval);
                self.pos.unmake_move(mv, &mut self.state_history);
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
