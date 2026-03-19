use std::str::FromStr;

use crate::chess::Error;

pub const NUM_RANKS: usize = 8;
pub const NUM_FILES: usize = 8;
pub const NUM_SQUARES: usize = NUM_RANKS * NUM_FILES;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl FromStr for File {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(Error::ParseError);
        }

        match s {
            "a" => Ok(Self::A),
            "b" => Ok(Self::B),
            "c" => Ok(Self::C),
            "d" => Ok(Self::D),
            "e" => Ok(Self::E),
            "f" => Ok(Self::F),
            "g" => Ok(Self::G),
            "h" => Ok(Self::H),
            _ => Err(Error::ParseError),
        }
    }
}

impl File {
    pub fn increment(&self, count: u8) -> Option<Self> {
        let index = (*self as u8) + count;
        if index >= 8 {
            None
        } else {
            Some(Self::from_index(index as usize).unwrap())
        }
    }

    pub fn decrement(&self, count: u8) -> Option<Self> {
        let index = (*self as i8) - count as i8;
        if index < 0 {
            None
        } else {
            Some(Self::from_index(index as usize).unwrap())
        }
    }

    fn from_index(index: usize) -> Result<Self, Error> {
        match index {
            0 => Ok(Self::A),
            1 => Ok(Self::B),
            2 => Ok(Self::C),
            3 => Ok(Self::D),
            4 => Ok(Self::E),
            5 => Ok(Self::F),
            6 => Ok(Self::G),
            7 => Ok(Self::H),
            _ => Err(Error::ParseError),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl Rank {
    pub fn increment(&self, count: u8) -> Option<Self> {
        let index = (*self as u8) + count;
        if index >= 8 {
            None
        } else {
            Some(Self::from_index(index as usize).unwrap())
        }
    }

    pub fn decrement(&self, count: u8) -> Option<Self> {
        let index = (*self as i8) - count as i8;
        if index < 0 {
            None
        } else {
            Some(Self::from_index(index as usize).unwrap())
        }
    }

    fn from_index(index: usize) -> Result<Self, Error> {
        match index {
            0 => Ok(Self::One),
            1 => Ok(Self::Two),
            2 => Ok(Self::Three),
            3 => Ok(Self::Four),
            4 => Ok(Self::Five),
            5 => Ok(Self::Six),
            6 => Ok(Self::Seven),
            7 => Ok(Self::Eight),
            _ => Err(Error::ParseError),
        }
    }
}

impl FromStr for Rank {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err(Error::ParseError);
        }

        match s {
            "1" => Ok(Self::One),
            "2" => Ok(Self::Two),
            "3" => Ok(Self::Three),
            "4" => Ok(Self::Four),
            "5" => Ok(Self::Five),
            "6" => Ok(Self::Six),
            "7" => Ok(Self::Seven),
            "8" => Ok(Self::Eight),
            _ => Err(Error::ParseError),
        }
    }
}

#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

impl Square {
    pub fn new(file: File, rank: Rank) -> Self {
        let index = (rank as u8 * NUM_FILES as u8) + file as u8;

        unsafe { std::mem::transmute(index) }
    }

    pub fn rank(&self) -> Rank {
        let index = (*self as u8) >> 3;
        match index {
            0 => Rank::One,
            1 => Rank::Two,
            2 => Rank::Three,
            3 => Rank::Four,
            4 => Rank::Five,
            5 => Rank::Six,
            6 => Rank::Seven,
            7 => Rank::Eight,
            _ => unreachable!(),
        }
    }

    pub fn file(&self) -> File {
        let index = (*self as u8) & 0b111;
        match index {
            0 => File::A,
            1 => File::B,
            2 => File::C,
            3 => File::D,
            4 => File::E,
            5 => File::F,
            6 => File::G,
            7 => File::H,
            _ => unreachable!(),
        }
    }
}

impl FromStr for Square {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 || !s.is_ascii() {
            return Err(Error::ParseError);
        }

        let (file, rank) = s.split_at(1);
        let file = File::from_str(file)?;
        let rank = Rank::from_str(rank)?;

        Ok(Square::new(file, rank))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_new() {
        let square = Square::new(File::E, Rank::Four);
        assert_eq!(square, Square::E4);
    }

    #[test]
    fn test_square_rank() {
        let square = Square::E4;
        assert_eq!(square.rank(), Rank::Four);
    }

    #[test]
    fn test_square_file() {
        let square = Square::E4;
        assert_eq!(square.file(), File::E);
    }

    #[test]
    fn test_square_from_str() {
        let square = Square::from_str("e4").unwrap();
        assert_eq!(square, Square::E4);
    }

    #[test]
    fn test_rank_from_str() {
        let rank = Rank::from_str("4").unwrap();
        assert_eq!(rank, Rank::Four);
    }

    #[test]
    fn test_rank_increment() {
        let rank = Rank::Four;
        let next_rank = rank.increment(2);
        assert_eq!(next_rank, Some(Rank::Six));

        let next_rank = rank.increment(5);
        assert_eq!(next_rank, None);
    }

    #[test]
    fn test_rank_decrement() {
        let rank = Rank::Four;
        let next_rank = rank.decrement(2);
        assert_eq!(next_rank, Some(Rank::Two));

        let next_rank = rank.decrement(5);
        assert_eq!(next_rank, None);
    }

    #[test]
    fn test_file_from_str() {
        let file = File::from_str("e").unwrap();
        assert_eq!(file, File::E);
    }

    #[test]
    fn test_file_increment() {
        let file = File::E;
        let next_file = file.increment(2);
        assert_eq!(next_file, Some(File::G));

        let next_file = file.increment(5);
        assert_eq!(next_file, None);
    }

    #[test]
    fn test_file_decrement() {
        let file = File::E;
        let next_file = file.decrement(2);
        assert_eq!(next_file, Some(File::C));

        let next_file = file.decrement(5);
        assert_eq!(next_file, None);
    }
}
