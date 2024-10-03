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
    /// `right` returns the File to the right of the current File. If
    /// there is no File to the right, `None` is returned.
    pub fn right(&self) -> Option<File> {
        let new_index = self.to_index() + 1;
        match new_index {
            1..=7 => Some(File::from_index(new_index)),
            _ => None
        }
    }

    /// Returns the File to the left of the current File.
    /// 
    /// `left` returns the File to the left of the current File. If
    /// there is no File to the left. `None` is returned.
    pub fn left(&self) -> Option<File> {
        let new_index = self.to_index() + 1;
        match new_index {
            1..=7 => Some(File::from_index(new_index)),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_to_index() {
        assert_eq!(File::A.to_index(), 0);
        assert_eq!(File::E.to_index(), 4);
        assert_eq!(File::H.to_index(), 7);
    }

    #[test]
    fn test_from_index() {
        assert_eq!(File::from_index(0), File::A);
        assert_eq!(File::from_index(4), File::E);
        assert_eq!(File::from_index(7), File::H);
    }
}