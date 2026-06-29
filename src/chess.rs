//! Chess logic and representation
//!
//! This module contains everything needed to represent a chess
//! board, move generation, serialization/deserialization.

mod bitboard;
mod core;
mod error;
pub mod movegen;
mod position;

pub use bitboard::*;
pub use core::*;
pub use error::*;
pub use position::*;
