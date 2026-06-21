use std::{fmt::Display, ops::Not, str::FromStr};

use crate::chess::Error;

pub const NUM_COLORS: usize = 2;
pub const NUM_PIECES: usize = 6;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn is_white(&self) -> bool {
        self == &Self::White
    }
}

impl Not for Color {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

impl FromStr for Color {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "w" => Ok(Self::White),
            "b" => Ok(Self::Black),
            _ => Err(Error::ParseError),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl TryFrom<usize> for PieceType {
    type Error = Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Pawn),
            1 => Ok(Self::Knight),
            2 => Ok(Self::Bishop),
            3 => Ok(Self::Rook),
            4 => Ok(Self::Queen),
            5 => Ok(Self::King),
            _ => Err(Error::InvalidIndex),
        }
    }
}

impl Display for PieceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let piece_char = match self {
            PieceType::Pawn => "p",
            PieceType::Knight => "n",
            PieceType::Bishop => "b",
            PieceType::Rook => "r",
            PieceType::Queen => "q",
            PieceType::King => "k",
        };

        write!(f, "{piece_char}")
    }
}

impl FromStr for PieceType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(Error::ParseError);
        }

        match s.to_lowercase().as_str() {
            "p" => Ok(Self::Pawn),
            "n" => Ok(Self::Knight),
            "b" => Ok(Self::Bishop),
            "r" => Ok(Self::Rook),
            "q" => Ok(Self::Queen),
            "k" => Ok(Self::King),
            _ => Err(Error::ParseError),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Piece {
    color: Color,
    piece_type: PieceType,
}

impl Piece {
    pub fn new(color: Color, piece_type: PieceType) -> Self {
        Self { color, piece_type }
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn piece_type(&self) -> PieceType {
        self.piece_type
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut piece_str = self.piece_type().to_string();
        if self.color().is_white() {
            piece_str = piece_str.to_uppercase();
        }

        write!(f, "{}", piece_str)
    }
}

impl FromStr for Piece {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(Error::ParseError);
        }

        let piece_type = s.parse()?;
        let color = s
            .chars()
            .nth(0)
            .map(|c| {
                if c.is_lowercase() {
                    Color::Black
                } else {
                    Color::White
                }
            })
            .ok_or(Error::ParseError)?;

        Ok(Self { color, piece_type })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_from_str() {
        let color = Color::from_str("w").unwrap();
        assert_eq!(Color::White, color);

        assert_eq!(Color::Black, Color::from_str("b").unwrap());

        assert!(Color::from_str("a").is_err());
    }

    #[test]
    fn test_piece_type_from_str() {
        let piece_type = PieceType::from_str("p").unwrap();
        assert_eq!(PieceType::Pawn, piece_type);

        assert_eq!(PieceType::King, PieceType::from_str("k").unwrap());

        assert!(PieceType::from_str("a").is_err());
    }

    #[test]
    fn test_piece_from_str() {
        let piece = Piece::from_str("P").unwrap();
        assert_eq!(Color::White, piece.color());
        assert_eq!(PieceType::Pawn, piece.piece_type());

        let piece = Piece::from_str("k").unwrap();
        assert_eq!(Color::Black, piece.color());
        assert_eq!(PieceType::King, piece.piece_type());

        assert!(Piece::from_str("Kp").is_err());
    }
}
