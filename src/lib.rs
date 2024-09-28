#![allow(unused, dead_code)]

use std::ops::Not;
use std::{io::Error, str::FromStr};

mod board;

enum Color {
    Black,
    White
}

impl Not for Color {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black
        }
    }
}

/// A chess piece.
enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}