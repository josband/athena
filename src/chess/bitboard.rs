use std::{
    fmt::{Debug, Display, Write},
    ops::{
        Add, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Index, IndexMut, Neg,
        Not, Shl, ShlAssign, Shr, ShrAssign,
    },
};

use crate::chess::{File, NUM_BITBOARDS, NUM_PIECES, NUM_SQUARES, Piece, Rank, Square};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North = 8,
    South = -8,
    East = 1,
    West = -1,
    NorthEast = 9,
    NorthWest = 7,
    SouthEast = -7,
    SouthWest = -9,
}

impl Direction {
    pub fn flip(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
            Direction::NorthEast => Direction::SouthWest,
            Direction::NorthWest => Direction::SouthEast,
            Direction::SouthEast => Direction::NorthWest,
            Direction::SouthWest => Direction::NorthEast,
        }
    }
}

impl Add for Direction {
    type Output = i32;

    fn add(self, rhs: Direction) -> Self::Output {
        self as i32 + rhs as i32
    }
}

/// Bit representation of a chess board.
///
/// There are many ways to represent a chess board, but using
/// bitboards is a common and efficient way to do so. Users benefit
/// from being able to perform many bitwise operations and the memory
/// overhead is significantly less. A 1 represents precense of a piece,
/// while a 0 represents an empty square. The means that the piece type
/// and color is ambiguous. Many bitboards are needed to represent a
/// position.
#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Bitboard(pub(crate) u64);

impl Bitboard {
    pub const EMPTY: Self = Self(0);

    pub fn shift(self, direction: Direction) -> Self {
        self.shift_n(direction, 1)
    }

    pub fn shift_n(self, direction: Direction, count: u64) -> Self {
        use Direction::*;

        let wrap_mask = !match direction {
            NorthWest | SouthWest | West => {
                let mut mask = Self::from(File::H);
                for _ in 0..count - 1 {
                    mask |= mask >> 1;
                }

                mask
            }
            NorthEast | SouthEast | East => {
                let mut mask = Self::from(File::A);
                for _ in 0..count - 1 {
                    mask |= mask << 1;
                }

                mask
            }
            _ => Self::EMPTY,
        };

        wrap_mask
            & match direction {
                North | NorthEast | NorthWest | East => self << (count * direction as u64),
                South | SouthEast | SouthWest | West => {
                    self >> (count * -(direction as i64) as u64)
                }
            }
    }

    pub fn pop_lsb(&mut self) -> Option<Square> {
        let lsb = self.lsb();
        if let Some(lsb) = lsb {
            self.0 ^= Bitboard::from(lsb).0;
        }

        lsb
    }

    pub fn lsb(&self) -> Option<Square> {
        if self.0.count_ones() == 0 {
            None
        } else {
            Square::from_lsf_index(self.0.trailing_zeros() as u8).ok()
        }
    }

    pub fn msb(&self) -> Option<Square> {
        if self.0.count_ones() == 0 {
            None
        } else {
            Square::from_lsf_index((NUM_SQUARES as u32 - self.0.leading_zeros() - 1) as u8).ok()
        }
    }
}

impl From<Square> for Bitboard {
    fn from(value: Square) -> Self {
        Bitboard(1 << (value as u64))
    }
}

impl From<Rank> for Bitboard {
    fn from(value: Rank) -> Self {
        Bitboard(
            0xff << (8 * match value {
                Rank::One => 0,
                Rank::Two => 1,
                Rank::Three => 2,
                Rank::Four => 3,
                Rank::Five => 4,
                Rank::Six => 5,
                Rank::Seven => 6,
                Rank::Eight => 7,
            }),
        )
    }
}

impl From<File> for Bitboard {
    fn from(value: File) -> Self {
        Bitboard(
            0x0101010101010101
                << match value {
                    File::A => 0,
                    File::B => 1,
                    File::C => 2,
                    File::D => 3,
                    File::E => 4,
                    File::F => 5,
                    File::G => 6,
                    File::H => 7,
                },
        )
    }
}

impl Debug for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Bitboard({:#x})", self.0))
    }
}

impl Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for byte_index in (0..8).rev() {
            for i in 0..8 {
                let mask = 1 << ((byte_index * 8) + i);
                if self.0 & mask != 0 {
                    f.write_char('X')?;
                } else {
                    f.write_char('·')?;
                }
            }

            if byte_index != 0 {
                f.write_char('\n')?;
            }
        }

        Ok(())
    }
}

impl Index<Piece> for [Bitboard; NUM_BITBOARDS] {
    type Output = Bitboard;

    fn index(&self, index: Piece) -> &Self::Output {
        let index = index.color() as usize * NUM_PIECES + index.piece_type() as usize;
        &self[index]
    }
}

impl IndexMut<Piece> for [Bitboard; NUM_BITBOARDS] {
    fn index_mut(&mut self, index: Piece) -> &mut Self::Output {
        let index = index.color() as usize * NUM_PIECES + index.piece_type() as usize;
        &mut self[index]
    }
}

impl Neg for Bitboard {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Bitboard(-(self.0 as i64) as u64)
    }
}

impl Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Bitboard(!self.0)
    }
}

impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Bitboard) -> Self::Output {
        Bitboard(self.0 | rhs.0)
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for Bitboard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 & rhs.0)
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitXor for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Bitboard(self.0 ^ rhs.0)
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl Shl<u64> for Bitboard {
    type Output = Self;

    fn shl(self, rhs: u64) -> Self::Output {
        Bitboard(self.0 << rhs)
    }
}

impl ShlAssign<u64> for Bitboard {
    fn shl_assign(&mut self, rhs: u64) {
        self.0 <<= rhs;
    }
}

impl Shr<u64> for Bitboard {
    type Output = Self;

    fn shr(self, rhs: u64) -> Self::Output {
        Bitboard(self.0 >> rhs)
    }
}

impl ShrAssign<u64> for Bitboard {
    fn shr_assign(&mut self, rhs: u64) {
        self.0 >>= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let board = Bitboard(0xF10000000000001A);
        assert_eq!(
            format!("{}", board),
            "X···XXXX\n········\n········\n········\n········\n········\n········\n·X·XX···"
        );
    }

    #[test]
    fn test_and() {
        let a = Bitboard(0b1111);
        let b = Bitboard(0b1010);
        assert_eq!(a & b, Bitboard(0b1010));
    }

    #[test]
    fn test_and_assign() {
        let mut a = Bitboard(0b1111);
        let b = Bitboard(0b1010);
        a &= b;
        assert_eq!(a, Bitboard(0b1010));
    }

    #[test]
    fn test_or() {
        let a = Bitboard(0b1110);
        let b = Bitboard(0b1010);
        assert_eq!(a | b, Bitboard(0b1110));
    }

    #[test]
    fn test_or_assign() {
        let mut a = Bitboard(0b1110);
        let b = Bitboard(0b1010);
        a |= b;
        assert_eq!(a, Bitboard(0b1110));
    }

    #[test]
    fn test_xor() {
        let a = Bitboard(0b1110);
        let b = Bitboard(0b1010);
        assert_eq!(a ^ b, Bitboard(0b0100));
    }

    #[test]
    fn test_xor_assign() {
        let mut a = Bitboard(0b1110);
        let b = Bitboard(0b1010);
        a ^= b;
        assert_eq!(a, Bitboard(0b0100));
    }

    #[test]
    fn test_shl() {
        let a = Bitboard(0b0111);
        assert_eq!(a << 1, Bitboard(0b1110));
    }

    #[test]
    fn test_shl_assign() {
        let mut a = Bitboard(0b0111);
        a <<= 1;
        assert_eq!(a, Bitboard(0b1110));
    }

    #[test]
    fn test_shr() {
        let a = Bitboard(0b1110);
        assert_eq!(a >> 2, Bitboard(0b0011));
    }

    #[test]
    fn test_shr_assign() {
        let mut a = Bitboard(0b1110);
        a >>= 2;
        assert_eq!(a, Bitboard(0b0011));
    }

    #[test]
    fn test_shr_msb() {
        let a = Bitboard(1 << 63);
        assert_eq!(a >> 4, Bitboard(0x0800000000000000));
    }

    #[test]
    fn test_from_file() {
        let bb = Bitboard::from(File::C);
        assert_eq!(bb, Bitboard(0x404040404040404));
    }

    #[test]
    fn test_from_rank() {
        let bb = Bitboard::from(Rank::Six);
        assert_eq!(bb, Bitboard(0xff0000000000));
    }

    #[test]
    fn test_shift() {
        let bb = Bitboard(0xa);
        assert_eq!(bb.shift(Direction::North), Bitboard(0xa00));
    }

    #[test]
    fn test_shift_n() {
        let bb = Bitboard(0x400000);
        assert_eq!(bb.shift_n(Direction::SouthWest, 2), Bitboard(0x10));

        let bb = Bitboard(0x100000000000);
        assert_eq!(bb.shift_n(Direction::SouthWest, 4), Bitboard(0x100));
    }

    #[test]
    fn test_shift_n_with_wrap() {
        let bb = Bitboard(0x20040000000);
        assert_eq!(
            bb.shift_n(Direction::NorthWest, 2),
            Bitboard(0x100000000000)
        );
    }

    #[test]
    fn test_shift_n_with_multiwrap() {
        let bb = Bitboard(0xe0);
        assert_eq!(bb.shift_n(Direction::NorthEast, 2), Bitboard(0x800000));
    }

    #[test]
    fn test_pop_lsb() {
        let mut bb = Bitboard(0x10);
        assert_eq!(bb.pop_lsb(), Some(Square::E1));
        assert_eq!(bb, Bitboard(0x0));
    }
}
