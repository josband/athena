#![allow(unused, dead_code)]
//! A chess engine as well as other utilities to create your own.

use std::ops::Not;
use std::{io::Error, str::FromStr};

pub mod board;

/// Represents the color of a piece.
/// 
/// Can be used to specify the behavior of a piece. Use cases include 
/// conversion to/from strings, determining direction of piece movement, 
/// which side is next to move, etc. 
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
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
/// 
/// Represents the different variants of chess pieces.
#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}