use std::{
    fmt::Debug,
    ops::{
        BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Index, IndexMut, Not, Shl,
        ShlAssign, Shr, ShrAssign,
    },
};

use crate::chess::{NUM_BITBOARDS, NUM_PIECES, Piece};

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
}

impl Debug for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:#018X}", self.0))
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
}
