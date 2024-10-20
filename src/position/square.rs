use crate::position::{File, Rank};

/// A square on the chess board.
///
/// The inner value of this struct is an index corresponding
/// to the specific square. 0 represents A1, followed by 1 representing B1 all the way to
/// 63 representing H8.
///
/// # Examples
///
/// ```
/// use athena::position::Square;
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

        Self(square)
    }

    /// Converst Square to its corresponding index
    pub fn to_index(&self) -> u8 {
        self.0
    }

    /// Creates a Square from a specified Rank and File.
    pub fn from_intersection(rank: Rank, file: File) -> Self {
        Self::new((rank.to_index() << 3 as u8) + file.to_index())
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

    /// Gets the Square above the current square.
    pub fn up(&self) -> Option<Self> {
        if self.get_rank() == Rank::Eighth {
            None
        } else {
            Some(Self::from_intersection(
                self.get_rank().up().unwrap(),
                self.get_file(),
            ))
        }
    }

    /// Gets the Square below the current square.
    pub fn down(&self) -> Option<Self> {
        if self.get_rank() == Rank::First {
            None
        } else {
            Some(Self::from_intersection(
                self.get_rank().down().unwrap(),
                self.get_file(),
            ))
        }
    }

    /// Gets the Square to the right of the current square.
    pub fn right(&self) -> Option<Self> {
        if self.get_file() == File::H {
            None
        } else {
            Some(Self::from_intersection(
                self.get_rank(),
                {
                    let this = &self.get_file();
                    let new_index = this.to_index() + 1;
                    match new_index {
                        1..=7 => Some(File::from_index(new_index)),
                        _ => None,
                    }
                }
                .unwrap(),
            ))
        }
    }

    /// Gets the Square to the left of the current square.
    pub fn left(&self) -> Option<Self> {
        if self.get_file() == File::A {
            None
        } else {
            Some(Self::from_intersection(
                self.get_rank(),
                self.get_file().left().unwrap(),
            ))
        }
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
