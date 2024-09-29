//! Structures and functions relating to the chess board

/// A rank (row) on a chess board.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum Rank {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth
}

impl Rank {
    /// Converts an integer value to the corresponding rank
    /// 
    /// # Panics
    /// `from_index` panics when the passed in index is greater than 7.
    pub fn from_index(index: u8) -> Self {
        match index {
            0 => Rank::First,
            1 => Rank::Second,
            2 => Rank::Third,
            3 => Rank::Fourth,
            4 => Rank::Fifth,
            5 => Rank::Sixth,
            6 => Rank::Seventh,
            7 => Rank::Eighth,
            _ => panic!("Unknown rank index: {index}")
        }
    }

    /// Converts the rank to it's corresponding index.
    /// 
    /// Indexes for ranks range from 0 to 7 with the first rank corresponding to 0 
    /// and 7 corresponding to the Eighth rank. 
    pub fn to_index(&self) -> u8 {
        *self as u8
    }

    /// Gets the rank above the current rank, if one exists.
    /// 
    /// `get_up` returns the rank above the current rank. If the
    /// given rank is the eighth rank, `None` is returned.
    pub fn get_up(&self) -> Option<Rank> {
        todo!()
    }

    /// Gets the rank below the current rank, if one exists.
    /// 
    /// `get_down` returns the rank above the current rank. If the 
    /// given rank is the eighth rank, `None` is returned. 
    pub fn get_down(&self) -> Option<Rank> {
        todo!()
    }
}

/// A file (column) on a chess board.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H
}

impl File {
    /// Converts an index to its corresponding File.
    /// 
    /// # Panics
    /// `from_index` panics when the index is outside the range
    /// 0 to 7.
    pub fn from_index(index: u8) -> Self {
        match index {
            0 => File::A,
            1 => File::B,
            2 => File::C,
            3 => File::D,
            4 => File::E,
            5 => File::F,
            6 => File::G,
            7 => File::H,
            _ => panic!("Unknown file index: {index}")
        }
    }

    /// Converts a File to its corresponding index.
    pub fn to_index(&self) -> u8 {
        *self as u8
    }

    /// Returns the File to the right of the current File.
    /// 
    /// `get_right` returns the File to the right of the current File. If
    /// there is no File to the right, `None` is returned.
    pub fn get_right(&self) -> Option<File> {
        todo!()
    }

    /// Returns the File to the left of the current File.
    /// 
    /// `get_left` returns the File to the left of the current File. If
    /// there is no File to the left. `None` is returned.
    pub fn get_left(&self) -> Option<File> {
        todo!()
    }
}

/// A square on the chess board. The inner value of this struct is an index corresponding
/// to the specific square. 0 represents A1, followed by 1 representing B1 all the way to
/// 63 representing H8.
/// 
/// # Examples
/// 
/// ```
/// use tango::board::Square;
/// let chess_square = Square::new(3);
/// 
/// assert_eq!(chess_square, Square::D1);
/// ```
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
pub struct Square(u8);

impl Square {
    /// Creates a new `Square` struct. 
    /// 
    /// # Panics
    /// 
    /// This code panics if the user inputs a value representing a square outside the 64 
    /// chess squares. In other words, `square` can only be within the bounds \[0, 63\].
    pub fn new(square: u8) -> Self {
        assert!(square <= 63);

        Square(square)
    }

    /// Creates a Square from a specified Rank and File.
    pub fn from_rank_and_file(rank: Rank, file: File) -> Self {
        Square::new((rank.to_index() << 3 as u8) + file.to_index())
    }    

    /// Returns the Rank of a Square.
    pub fn get_rank(&self) -> Rank {
        Rank::from_index(self.0 >> 3)
    }

    /// Returns the File of a Square.
    pub fn get_file(&self) -> File {
        File::from_index(self.0 & 7)
    }
    
    /// Returns the Rank and File of a Square.
    pub fn get_rank_and_file(&self) -> (Rank, File) {
        (self.get_rank(), self.get_file())
    }

    pub const A1: Square = Square(0);
    pub const B1: Square = Square(1);
    pub const C1: Square = Square(2);
    pub const D1: Square = Square(3);
    pub const E1: Square = Square(4);
    pub const F1: Square = Square(5);
    pub const G1: Square = Square(6);
    pub const H1: Square = Square(7);
    pub const A2: Square = Square(8);
    pub const B2: Square = Square(9);
    pub const C2: Square = Square(10);
    pub const D2: Square = Square(11);
    pub const E2: Square = Square(12);
    pub const F2: Square = Square(13);
    pub const G2: Square = Square(14);
    pub const H2: Square = Square(15);
    pub const A3: Square = Square(16);
    pub const B3: Square = Square(17);
    pub const C3: Square = Square(18);
    pub const D3: Square = Square(19);
    pub const E3: Square = Square(20);
    pub const F3: Square = Square(21);
    pub const G3: Square = Square(22);
    pub const H3: Square = Square(23);
    pub const A4: Square = Square(24);
    pub const B4: Square = Square(25);
    pub const C4: Square = Square(26);
    pub const D4: Square = Square(27);
    pub const E4: Square = Square(28);
    pub const F4: Square = Square(29);
    pub const G4: Square = Square(30);
    pub const H4: Square = Square(31);
    pub const A5: Square = Square(32);
    pub const B5: Square = Square(33);
    pub const C5: Square = Square(34);
    pub const D5: Square = Square(35);
    pub const E5: Square = Square(36);
    pub const F5: Square = Square(37);
    pub const G5: Square = Square(38);
    pub const H5: Square = Square(39);
    pub const A6: Square = Square(40);
    pub const B6: Square = Square(41);
    pub const C6: Square = Square(42);
    pub const D6: Square = Square(43);
    pub const E6: Square = Square(44);
    pub const F6: Square = Square(45);
    pub const G6: Square = Square(46);
    pub const H6: Square = Square(47);
    pub const A7: Square = Square(48);
    pub const B7: Square = Square(49);
    pub const C7: Square = Square(50);
    pub const D7: Square = Square(51);
    pub const E7: Square = Square(52);
    pub const F7: Square = Square(53);
    pub const G7: Square = Square(54);
    pub const H7: Square = Square(55);
    pub const A8: Square = Square(56);
    pub const B8: Square = Square(57);
    pub const C8: Square = Square(58);
    pub const D8: Square = Square(59);
    pub const E8: Square = Square(60);
    pub const F8: Square = Square(61);
    pub const G8: Square = Square(62);
    pub const H8: Square = Square(63);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rank_index() {
        assert_eq!(Rank::First.to_index(), 0);
        assert_eq!(Rank::Fifth.to_index(), 4);
        assert_eq!(Rank::Eighth.to_index(), 7);
    }

    #[test]
    fn test_file_index() {
        assert_eq!(File::A.to_index(), 0);
        assert_eq!(File::E.to_index(), 4);
        assert_eq!(File::H.to_index(), 7);
    }

    #[test]
    fn test_rank_from_index() {
        assert_eq!(Rank::from_index(0), Rank::First);
        assert_eq!(Rank::from_index(4), Rank::Fifth);
        assert_eq!(Rank::from_index(7), Rank::Eighth);
    }
    
    #[test]
    fn test_file_from_index() {
        assert_eq!(File::from_index(0), File::A);
        assert_eq!(File::from_index(4), File::E);
        assert_eq!(File::from_index(7), File::H);
    }
}