use std::{
    ops::{Index, IndexMut},
    str::FromStr,
};

use crate::chess::{
    Bitboard, Color, File, NUM_COLORS, NUM_PIECES, NUM_RANKS, Piece, PieceType, Rank, Square,
    error::Error, movegen::Move,
};

pub(crate) const NUM_BITBOARDS: usize = NUM_COLORS * NUM_PIECES;
pub const FEN_RANK_SEPARATOR: char = '/';
pub const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CastlingRights {
    All,
    QueenSide,
    KingSide,
    None,
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

/// Representation of a chess position.
///
/// A [chess position](https://www.chessprogramming.org/Chess_Position) is defined as the current
/// state of an entire chess board during a particular point in the game. One can think of it as
/// a snapshot or picture of the board at a given moment. This includes piece placement, side to
/// move, castling rights, an optional en passant square, and the half-move clock. Move history
/// is not a part of the position itself and is tracked as part of an entire game. Practically
/// all rules can be applied based on the position alone. The only rule that cannot be applied
/// from a position is the determination of three fold repititions.
#[allow(unused)]
pub struct Position {
    bitboards: [Bitboard; NUM_BITBOARDS],
    side_to_move: Color,
    castling_rights: [CastlingRights; NUM_COLORS],
    en_passant_square: Option<Square>,
    half_move_clock: u8,
}

impl Position {
    pub fn side_to_move(&self) -> Color {
        self.side_to_move
    }

    pub fn occupied(&self) -> Bitboard {
        self.bitboards
            .iter()
            .fold(Bitboard::EMPTY, |acc, bb| acc | *bb)
    }

    pub fn empty_squares(&self) -> Bitboard {
        !self.occupied()
    }

    /// Fetches all instances of a given piece on the board.
    pub fn piece(&self, piece: Piece) -> Bitboard {
        self.bitboards[piece]
    }

    pub fn has_en_passant(&self) -> bool {
        self.en_passant_square.is_some()
    }

    pub fn en_passant_square(&self) -> Option<Square> {
        self.en_passant_square
    }

    pub fn castling_rights(&self, color: Color) -> CastlingRights {
        self.castling_rights[color]
    }

    pub fn color_pieces(&self, color: Color) -> Bitboard {
        let range = if color.is_white() {
            0..NUM_PIECES
        } else {
            NUM_PIECES..NUM_BITBOARDS
        };
        let mut combined_pieces = Bitboard::EMPTY;
        for i in range {
            combined_pieces |= self.bitboards[i];
        }

        combined_pieces
    }

    pub fn get_piece_at(&self, square: Square) -> Option<Piece> {
        let mut piece_opt = None;
        let square_bb = Bitboard::from(square);
        for (i, bb) in self.bitboards.iter().enumerate() {
            if *bb & square_bb != Bitboard::EMPTY {
                let color = if i < NUM_PIECES {
                    Color::White
                } else {
                    Color::Black
                };

                piece_opt = Some(Piece::new(
                    color,
                    PieceType::try_from(i % NUM_BITBOARDS).ok()?,
                ));
                break;
            }
        }

        piece_opt
    }

    /// Attempts to make a move.
    ///
    /// Takes a psuedo-legal move and attempts to update board state according to the moves. If
    /// the move does not align with board state (ex. No piece exists at the source square), an
    /// error is returned.
    pub fn make_move(&mut self, _mv: Move) -> bool {
        false
    }

    /// Attempts to unmake a move
    ///
    /// Takes a psuedo-legal move and attempts to update board state according to undoing the moves. If
    /// the move does not align with board state, a panic! will occur.
    pub fn unmake_move(&mut self, _mv: Move) {
        todo!()
    }

    pub fn is_checked(&self) -> bool {
        todo!()
    }
}

impl FromStr for Position {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_ascii() {
            return Err(Error::InvalidFen);
        }

        let components: Vec<&str> = s.split_whitespace().collect();
        if components.len() != 6 {
            return Err(Error::InvalidFen);
        }

        let bitboards = parse_fen_board(components[0])?;
        let side_to_move: Color = components[1].parse()?;
        let castling_rights = if components[2] == "-" {
            [CastlingRights::None; NUM_COLORS]
        } else {
            let rights_str = components[2];
            let split_index = rights_str
                .char_indices()
                .find(|(_, c)| c.is_lowercase())
                .map(|(i, _)| i)
                .unwrap_or(rights_str.len());
            let (black_rights, white_rights) = rights_str.split_at(split_index);

            [white_rights.parse()?, black_rights.parse()?]
        };

        // TODO: Add validation checks of the en passant square and board
        let en_passant_square: Option<Square> = match components[3] {
            "-" => None,
            _ => Some(components[3].parse().map_err(|_| Error::InvalidFen)?),
        };

        let half_move_clock: u8 = components[4].parse().map_err(|_| Error::InvalidFen)?;

        // The full move clock is not needed for basic position parsing, but still good to check
        let _: usize = components[5].parse().map_err(|_| Error::InvalidFen)?;

        Ok(Self {
            bitboards,
            side_to_move,
            castling_rights,
            en_passant_square,
            half_move_clock,
        })
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::from_str(STARTING_FEN).expect("Failed to parse starting FEN position")
    }
}

fn parse_fen_board(fen_board: &str) -> Result<[Bitboard; NUM_BITBOARDS], Error> {
    let piece_placement: Vec<&str> = fen_board.split(FEN_RANK_SEPARATOR).collect();
    if piece_placement.len() != NUM_RANKS {
        return Err(Error::InvalidFen);
    }

    let mut bitboards = [Bitboard::EMPTY; NUM_BITBOARDS];
    let mut rank_opt = Some(Rank::Eight);
    for &rank_str in piece_placement.iter() {
        let rank = rank_opt.ok_or(Error::InvalidFen)?;
        let mut file_opt = Some(File::A);
        for c in rank_str.chars() {
            let file = file_opt.ok_or(Error::InvalidFen)?;
            if c.is_numeric() {
                let count = c.to_digit(10).ok_or(Error::ParseError)?;
                file_opt = file.right_n(count as u8);
            } else if c.is_alphabetic() {
                let piece: Piece = c.to_string().parse()?;
                bitboards[piece] |= Bitboard(1 << Square::new(file, rank) as u64);
                file_opt = file.right_n(1);
            } else {
                return Err(Error::InvalidFen);
            }
        }

        rank_opt = rank.down_n(1);
    }

    Ok(bitboards)
}

#[cfg(test)]
mod tests {
    use crate::chess::position::*;

    #[test]
    fn test_starting_parse() {
        let p = Position::default();

        let expected_bb: [Bitboard; NUM_BITBOARDS] = [
            Bitboard(0x000000000000FF00),
            Bitboard(0x0000000000000042),
            Bitboard(0x0000000000000024),
            Bitboard(0x0000000000000081),
            Bitboard(0x0000000000000008),
            Bitboard(0x0000000000000010),
            Bitboard(0x00FF000000000000),
            Bitboard(0x4200000000000000),
            Bitboard(0x2400000000000000),
            Bitboard(0x8100000000000000),
            Bitboard(0x0800000000000000),
            Bitboard(0x1000000000000000),
        ];

        assert_eq!(Color::White, p.side_to_move);
        assert_eq!(0, p.half_move_clock);
        assert_eq!(None, p.en_passant_square);
        assert_eq!([CastlingRights::All; NUM_COLORS], p.castling_rights);
        assert_eq!(expected_bb, p.bitboards);
    }

    #[test]
    fn test_midgame_parse() {
        let mid_game_fen = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq e4 1 2";
        let p: Position = mid_game_fen.parse().unwrap();

        let expected_bb: [Bitboard; NUM_BITBOARDS] = [
            Bitboard(0x000000001000EF00),
            Bitboard(0x0000000000200002),
            Bitboard(0x0000000000000024),
            Bitboard(0x0000000000000081),
            Bitboard(0x0000000000000008),
            Bitboard(0x0000000000000010),
            Bitboard(0x00FB000400000000),
            Bitboard(0x4200000000000000),
            Bitboard(0x2400000000000000),
            Bitboard(0x8100000000000000),
            Bitboard(0x0800000000000000),
            Bitboard(0x1000000000000000),
        ];

        assert_eq!(Color::Black, p.side_to_move);
        assert_eq!(1, p.half_move_clock);
        assert_eq!(Some(Square::E4), p.en_passant_square);
        assert_eq!([CastlingRights::All; NUM_COLORS], p.castling_rights);
        assert_eq!(expected_bb, p.bitboards);
    }

    #[test]
    fn test_endgame_parse() {
        let mid_game_fen = "5k2/ppp5/4P3/3R3p/6P1/1K2Nr3/PP3P2/8 b - - 1 32";
        let p: Position = mid_game_fen.parse().unwrap();

        let expected_bb: [Bitboard; NUM_BITBOARDS] = [
            Bitboard(0x0000100040002300),
            Bitboard(0x0000000000100000),
            Bitboard(0x00),
            Bitboard(0x0000000800000000),
            Bitboard(0x0000000000000000),
            Bitboard(0x0000000000020000),
            Bitboard(0x0007008000000000),
            Bitboard(0x00),
            Bitboard(0x00),
            Bitboard(0x0000000000200000),
            Bitboard(0x00),
            Bitboard(0x2000000000000000),
        ];

        assert_eq!(Color::Black, p.side_to_move);
        assert_eq!(1, p.half_move_clock);
        assert_eq!(None, p.en_passant_square);
        assert_eq!([CastlingRights::None; NUM_COLORS], p.castling_rights);
        assert_eq!(expected_bb, p.bitboards);
    }
}
