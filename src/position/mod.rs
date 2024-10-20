//! Structures and functions relating to a chess position.
//! 
//! A chess position consists of many components. While the board itself and the 
//! pieces which sit it are a part of a position, a posigtion is not exclusively 
//! that. Other components of a position include a side to move, castling rights, 
//! and other components necessary to encapsulating the an  

pub use self::bitboard::BitBoard;
pub use self::file::File;
pub use self::rank::Rank;
pub use self::square::Square;

mod bitboard;
mod file;
mod rank;
mod square;

use crate::{Color, NUM_COLORS, NUM_PIECES};

pub struct Position {
    white_pieces: [BitBoard; NUM_PIECES],
    black_pieces: [BitBoard; NUM_PIECES],
    side_to_move: Color,
}
