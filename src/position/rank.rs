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
    Eighth,
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
            _ => panic!("Unknown rank index: {index}"),
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
    /// `up` returns the rank above the current rank. If the
    /// given rank is the eighth rank, `None` is returned.
    pub fn up(&self) -> Option<Self> {
        match self {
            Self::Eighth => None,
            _ => Some(Self::from_index(self.to_index() + 1)),
        }
    }

    /// Gets the rank below the current rank, if one exists.
    ///
    /// `down` returns the rank above the current rank. If the
    /// given rank is the eighth rank, `None` is returned.
    pub fn down(&self) -> Option<Self> {
        match self {
            Self::First => None,
            _ => Some(Self::from_index(self.to_index() - 1)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_index() {
        assert_eq!(Rank::First.to_index(), 0);
        assert_eq!(Rank::Fifth.to_index(), 4);
        assert_eq!(Rank::Eighth.to_index(), 7);
    }

    #[test]
    fn test_from_index() {
        assert_eq!(Rank::from_index(0), Rank::First);
        assert_eq!(Rank::from_index(4), Rank::Fifth);
        assert_eq!(Rank::from_index(7), Rank::Eighth);
    }

    #[test]
    fn test_up_some() {
        assert_eq!(Rank::First.up(), Some(Rank::Second));
    }

    #[test]
    fn test_up_none() {
        assert!(Rank::Eighth.up().is_none());
    }

    #[test]
    fn test_down_some() {
        assert_eq!(Rank::Eighth.down(), Some(Rank::Seventh));
    }

    #[test]
    fn test_down_none() {
        assert!(Rank::First.down().is_none());
    }
}
