use std::{
    fmt::Display,
    ops::{Add, Index, IndexMut, Not, Sub},
    str::FromStr,
};

use crate::chess::{Bitboard, Error};

pub const NUM_RANKS: usize = 8;
pub const NUM_FILES: usize = 8;
pub const NUM_SQUARES: usize = NUM_RANKS * NUM_FILES;
pub const NUM_COLORS: usize = 2;
pub const NUM_PIECES: usize = 6;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
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

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            File::A => 'a',
            File::B => 'b',
            File::C => 'c',
            File::D => 'd',
            File::E => 'e',
            File::F => 'f',
            File::G => 'g',
            File::H => 'h',
        };

        write!(f, "{c}")
    }
}

impl File {
    pub fn right_n(&self, count: u8) -> Option<Self> {
        let index = (*self as u8) + count;
        if index >= 8 {
            None
        } else {
            Some(Self::from_index(index as usize).unwrap())
        }
    }

    pub fn left_n(&self, count: u8) -> Option<Self> {
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

    pub(crate) fn values() -> FileIterator {
        FileIterator::new()
    }
}

pub struct FileIterator {
    next: Option<File>,
}

impl FileIterator {
    fn new() -> Self {
        FileIterator {
            next: Some(File::A),
        }
    }
}

impl Iterator for FileIterator {
    type Item = File;

    fn next(&mut self) -> Option<Self::Item> {
        let val = self.next.take();
        if let Some(file) = val {
            self.next = file.right_n(1);
        }

        val
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
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

impl Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Rank::One => '1',
            Rank::Two => '2',
            Rank::Three => '3',
            Rank::Four => '4',
            Rank::Five => '5',
            Rank::Six => '6',
            Rank::Seven => '7',
            Rank::Eight => '8',
        };

        write!(f, "{c}")
    }
}

impl Rank {
    pub fn up_n(&self, count: u8) -> Option<Self> {
        let index = (*self as u8) + count;
        if index >= 8 {
            None
        } else {
            Some(Self::from_index(index as usize).unwrap())
        }
    }

    pub fn down_n(&self, count: u8) -> Option<Self> {
        let index = (*self as i8) - count as i8;
        if index < 0 {
            None
        } else {
            Some(Self::from_index(index as usize).unwrap())
        }
    }

    pub fn distance(&self, other: &Self) -> i8 {
        *self as i8 - *other as i8
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

    pub fn values() -> RankIterator {
        RankIterator::new(Rank::One)
    }

    pub fn values_from(start: Rank) -> RankIterator {
        RankIterator::new(start)
    }
}

pub struct RankIterator {
    next: Option<Rank>,
}

impl RankIterator {
    fn new(start: Rank) -> Self {
        Self { next: Some(start) }
    }
}

impl Iterator for RankIterator {
    type Item = Rank;

    fn next(&mut self) -> Option<Self::Item> {
        let curr_rank_opt = self.next.take();
        if let Some(curr_rank) = curr_rank_opt {
            self.next = curr_rank.up_n(1);
        }

        curr_rank_opt
    }
}

impl DoubleEndedIterator for RankIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        let curr_rank_opt = self.next.take();
        if let Some(curr_rank) = curr_rank_opt {
            self.next = curr_rank.down_n(1);
        }

        curr_rank_opt
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

pub struct SquaresIter {
    rank: Option<Rank>,
    file: Option<File>,
}

impl Iterator for SquaresIter {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.rank, self.file) {
            (Some(r), Some(f)) => {
                let s = Square::new(f, r);
                self.file = f.right_n(1);
                if self.file.is_none() {
                    self.rank = r.up_n(1);
                    self.file = Some(File::A);
                }

                Some(s)
            }
            _ => None,
        }
    }
}

#[rustfmt::skip]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
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
        let index = self.lsf_index() >> 3;
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
        let index = self.lsf_index() & 0b111;
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

    pub fn north(&self) -> Option<Self> {
        if let Rank::Eight = self.rank() {
            return None;
        }

        Some(Self::new(self.file(), self.rank().up_n(1).unwrap()))
    }

    pub fn south(&self) -> Option<Self> {
        if let Rank::One = self.rank() {
            return None;
        }

        Some(Self::new(self.file(), self.rank().down_n(1).unwrap()))
    }

    pub fn east(&self) -> Option<Self> {
        if let File::H = self.file() {
            return None;
        }

        Some(Self::new(self.file().right_n(1).unwrap(), self.rank()))
    }

    pub fn west(&self) -> Option<Self> {
        if let File::A = self.file() {
            return None;
        }

        Some(Self::new(self.file().left_n(1).unwrap(), self.rank()))
    }

    /// Returns the index of the square using Least Significant File indexing.
    pub fn lsf_index(&self) -> usize {
        *self as usize
    }

    pub fn from_lsf_index(index: u8) -> Result<Self, Error> {
        if index > 63 {
            Err(Error::InvalidIndex)
        } else {
            let s = unsafe { std::mem::transmute::<u8, Self>(index) };
            Ok(s)
        }
    }

    pub fn values() -> SquaresIter {
        SquaresIter {
            rank: Some(Rank::One),
            file: Some(File::A),
        }
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.file(), self.rank())
    }
}

impl Sub<i32> for Square {
    type Output = Option<Self>;

    fn sub(self, rhs: i32) -> Self::Output {
        self + -rhs
    }
}

impl Add<i32> for Square {
    type Output = Option<Self>;

    fn add(self, rhs: i32) -> Self::Output {
        if rhs.is_negative() {
            Bitboard::from(self) >> (-rhs as u64)
        } else {
            Bitboard::from(self) << (rhs as u64)
        }
        .pop_lsb()
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
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
#[repr(u8)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CastlingRights {
    All = 0b11,
    QueenSide = 0b01,
    KingSide = 0b10,
    None = 0b00,
}

impl CastlingRights {
    /// Updates castling rights to not include a given set of rights
    pub fn downgrade(&self, downgrade: CastlingRights) -> CastlingRights {
        let self_byte = *self as u8;
        let other_byte = downgrade as u8;
        if downgrade == CastlingRights::None || self_byte & other_byte == 0 {
            return *self;
        }

        let result_byte = self_byte ^ (self_byte & other_byte);

        unsafe { std::mem::transmute(result_byte) }
    }
}

impl Index<Color> for [CastlingRights; NUM_COLORS] {
    type Output = CastlingRights;

    fn index(&self, index: Color) -> &Self::Output {
        let color_index = index as usize;
        &self[color_index]
    }
}

impl IndexMut<Color> for [CastlingRights; NUM_COLORS] {
    fn index_mut(&mut self, index: Color) -> &mut Self::Output {
        let color_index = index as usize;
        &mut self[color_index]
    }
}

impl FromStr for CastlingRights {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "kq" => Ok(Self::All),
            "q" => Ok(Self::QueenSide),
            "k" => Ok(Self::KingSide),
            "" => Ok(Self::None),
            _ => Err(Error::ParseError),
        }
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
        let next_rank = rank.up_n(2);
        assert_eq!(next_rank, Some(Rank::Six));

        let next_rank = rank.up_n(5);
        assert_eq!(next_rank, None);
    }

    #[test]
    fn test_rank_decrement() {
        let rank = Rank::Four;
        let next_rank = rank.down_n(2);
        assert_eq!(next_rank, Some(Rank::Two));

        let next_rank = rank.down_n(5);
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
        let next_file = file.right_n(2);
        assert_eq!(next_file, Some(File::G));

        let next_file = file.right_n(5);
        assert_eq!(next_file, None);
    }

    #[test]
    fn test_file_decrement() {
        let file = File::E;
        let next_file = file.left_n(2);
        assert_eq!(next_file, Some(File::C));

        let next_file = file.left_n(5);
        assert_eq!(next_file, None);
    }

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
