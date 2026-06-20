use std::{
    fmt::Display,
    ops::{Index, IndexMut},
    str::FromStr,
};

use crate::chess::{
    Bitboard, Color, File, NUM_COLORS, NUM_FILES, NUM_PIECES, NUM_RANKS, Piece, PieceType, Rank,
    Square,
    error::Error,
    movegen::{Move, MoveKind, attack_mask, bishop_attacks, pawn_attack_mask, rook_attacks},
};

pub const FEN_RANK_SEPARATOR: char = '/';
pub const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
pub(crate) const NUM_BITBOARDS: usize = NUM_COLORS * NUM_PIECES;

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

/// State that cannot be recovered by the inverse of a move alone.
#[derive(Debug, PartialEq, Eq)]
pub struct State {
    castling_rights: [CastlingRights; NUM_COLORS],
    en_passant_square: Option<Square>,
    half_move_clock: u8,
    captured_piece: Option<Piece>,
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
#[derive(Debug)]
pub struct Position {
    bitboards: [Bitboard; NUM_BITBOARDS],
    side_to_move: Color,
    castling_rights: [CastlingRights; NUM_COLORS],
    en_passant_square: Option<Square>,
    half_move_clock: u8,
}

const RANK_DIVIDER: &str = "+---+---+---+---+---+---+---+---+";
const FILE_LABEL_TEMPLATE: &str = "  a   b   c   d   e   f   g   h  ";
macro_rules! rank_piece_template {
    () => {
        "| {} | {} | {} | {} | {} | {} | {} | {} | {}"
    };
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", RANK_DIVIDER)?;
        let mut piece_strs: [String; NUM_FILES] = std::array::from_fn(|_| String::new());
        for rank in Rank::values_from(Rank::Eight).rev() {
            for file in File::values() {
                let piece_str = self
                    .get_piece_at(&Square::new(file, rank))
                    .map(|x| x.to_string())
                    .unwrap_or_else(|| " ".to_string());

                piece_strs[file as usize] = piece_str;
            }

            writeln!(
                f,
                rank_piece_template!(),
                piece_strs[0],
                piece_strs[1],
                piece_strs[2],
                piece_strs[3],
                piece_strs[4],
                piece_strs[5],
                piece_strs[6],
                piece_strs[7],
                rank as u8 + 1
            )?;
            writeln!(f, "{}", RANK_DIVIDER)?;
        }

        writeln!(f, "{}", FILE_LABEL_TEMPLATE)
    }
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

    pub fn get_piece_at(&self, square: &Square) -> Option<Piece> {
        let mut piece_opt = None;
        let square_bb = Bitboard::from(*square);
        for (i, bb) in self.bitboards.iter().enumerate() {
            if *bb & square_bb != Bitboard::EMPTY {
                let color = if i < NUM_PIECES {
                    Color::White
                } else {
                    Color::Black
                };

                piece_opt = Some(Piece::new(color, PieceType::try_from(i % NUM_PIECES).ok()?));
                break;
            }
        }

        piece_opt
    }

    /// Attempts to make a move.
    ///
    /// Takes a psuedo-legal move and attempts to update board state according to the moves. If
    /// the move is not pseudo-legal, `make_move` can panic (for now).
    pub fn make_move(&mut self, mv: Move, history: &mut Vec<State>) -> bool {
        let mut is_legal = true;
        let mut saved_state = self.board_state();

        let us = self.side_to_move();
        let them = !us;
        let from = mv.from_sq();
        let to = mv.to_sq();
        let kind = mv.kind();
        let moved_piece = self.get_piece_at(&from).expect("no piece at from square");
        let captured_piece = self.get_piece_at(&to);

        debug_assert_eq!(
            moved_piece.color(),
            us,
            "moved piece is not of the expected color"
        );

        // Confirm we aren't doing a castle in check to avoid making updates
        if mv.kind() == MoveKind::Castle && self.is_checked(us) {
            return false;
        }

        // Update the moved piece bitboard
        let moved_piece_bb = self.piece_mut(moved_piece);
        *moved_piece_bb ^= from.into();
        *moved_piece_bb ^= to.into();

        // Handle special move cases
        match kind {
            MoveKind::Capture => {
                let captured_piece = captured_piece
                    .expect("captured piece does not exist on internal board representation");
                debug_assert_eq!(
                    captured_piece.color(),
                    them,
                    "Captured piece is not of the opposite color"
                );

                let captured_piece_bb = self.piece_mut(captured_piece);
                *captured_piece_bb ^= to.into();
                saved_state.captured_piece = Some(captured_piece);
            }
            MoveKind::EnPassant => {
                debug_assert_ne!(self.en_passant_square(), None);
                let ep_target = self
                    .en_passant_square()
                    .expect("no En Passant target for pseudo-legal move to be possible");
                let capture_square = Square::new(ep_target.file(), from.rank());
                *self.piece_mut(Piece::new(them, PieceType::Pawn)) ^= capture_square.into();
                saved_state.captured_piece = Some(Piece::new(them, PieceType::Pawn));
            }
            MoveKind::Castle => {
                let king_side = to.file() == File::G;
                let rook_destination =
                    Square::new(if king_side { File::F } else { File::D }, to.rank());
                let rooks_bb = self.piece_mut(Piece::new(us, PieceType::Rook));
                let mut rook_mask = (Bitboard(1) << (if king_side { 7 } else { 0 }))
                    << if us.is_white() { 0 } else { 56 };
                rook_mask |= rook_destination.into();
                *rooks_bb ^= rook_mask;
                self.castling_rights[us] = CastlingRights::None;
                if self.is_attacked(rook_destination, them) {
                    is_legal = false;
                }
            }
            MoveKind::Promotion(piece_type) => {
                *self.piece_mut(moved_piece) ^= to.into();
                *self.piece_mut(Piece::new(us, piece_type)) ^= to.into();

                // We moved the pawn which may be causing issues
                if from.file() != to.file() {
                    let captured_piece = captured_piece
                        .expect("captured EP/Promo piece not in board representation");

                    *self.piece_mut(captured_piece) ^= to.into();
                }

                if captured_piece.is_some() {
                    saved_state.captured_piece = captured_piece;
                }
            }
            _ => (),
        }

        // Update en-passant square
        self.en_passant_square = if PieceType::Pawn == moved_piece.piece_type() {
            let enemy_pawns = self.piece(Piece::new(them, PieceType::Pawn));
            if from.rank().distance(&to.rank()).abs() == 2
                && (enemy_pawns
                    & (to.east().map(Bitboard::from).unwrap_or(Bitboard::EMPTY)
                        | to.west().map(Bitboard::from).unwrap_or(Bitboard::EMPTY))
                    != Bitboard::EMPTY)
            {
                Some(if us.is_white() {
                    from.north().unwrap()
                } else {
                    from.south().unwrap()
                })
            } else {
                None
            }
        } else {
            None
        };

        // Update castling rights/halfmove clock, if king/rook moved
        match moved_piece.piece_type() {
            PieceType::King => {
                self.castling_rights[us] = CastlingRights::None;
            }
            PieceType::Rook => {
                self.remove_rights_for_rook(us, from);
            }
            _ => (),
        }

        // Update castling rights after capture
        if let Some(p) = captured_piece
            && p.piece_type() == PieceType::Rook
        {
            self.remove_rights_for_rook(them, to);
        }

        // Halfmove clock is reset by any pawn move and/or capture
        self.half_move_clock =
            if mv.kind() == MoveKind::Capture || moved_piece.piece_type() == PieceType::Pawn {
                0
            } else {
                self.half_move_clock + 1
            };

        self.side_to_move = !self.side_to_move;

        history.push(saved_state);
        if is_legal && self.is_checked(us) {
            is_legal = false;
        }

        if !is_legal {
            self.unmake_move(mv, history);
        }

        is_legal
    }

    /// Returns whether a specified side is in check
    pub fn is_checked(&self, side: Color) -> bool {
        let king_square = self
            .piece(Piece::new(side, PieceType::King))
            .pop_lsb()
            .expect("no king on board");
        self.is_attacked(king_square, !side)
    }

    /// Attempts to unmake a move
    ///
    /// Takes a psuedo-legal move and attempts to update board state according to undoing the moves. If
    /// the move does not align with board state, a panic! will occur.
    pub fn unmake_move(&mut self, mv: Move, history: &mut Vec<State>) {
        if history.is_empty() {
            return;
        }

        let state = history.pop().expect("history length of 0");
        let them = self.side_to_move();
        let us = !them;
        let from = mv.to_sq();
        let to = mv.from_sq();
        let moved_piece = self
            .get_piece_at(&from)
            .expect("no piece at moved location");

        // Move piece back to original square
        let moved_piece_bb = self.piece_mut(moved_piece);
        *moved_piece_bb ^= from.into();
        *moved_piece_bb ^= to.into();

        // Handle special case moves
        match mv.kind() {
            MoveKind::EnPassant => {
                debug_assert!(state.en_passant_square.is_some());
                debug_assert_eq!(state.en_passant_square.unwrap(), from);
                debug_assert!(state.captured_piece.is_some());
                debug_assert_eq!(
                    state.captured_piece.unwrap(),
                    Piece::new(them, PieceType::Pawn)
                );

                let ep_rank = if us.is_white() {
                    Rank::Five
                } else {
                    Rank::Four
                };

                let captured_piece = state
                    .captured_piece
                    .expect("no taken pawn stored for En Passant");
                let en_passant_square = Square::new(from.file(), ep_rank);
                *self.piece_mut(captured_piece) ^= en_passant_square.into();
            }
            MoveKind::Castle => {
                // Just need to restore rook back to home file
                let king_file = from.file();
                let king_rank = from.rank();
                debug_assert!(
                    (king_rank == Rank::One && us.is_white())
                        || (king_rank == Rank::Eight && !us.is_white())
                );
                let (rook_home_file, rook_castle_file) = if king_file == File::C {
                    (File::A, File::D)
                } else {
                    (File::H, File::F)
                };

                let rook_castle_sq = Square::new(rook_castle_file, king_rank);
                let rook_home_sq = Square::new(rook_home_file, king_rank);
                let rooks_bb = self.piece_mut(Piece::new(us, PieceType::Rook));
                *rooks_bb ^= rook_castle_sq.into();
                *rooks_bb ^= rook_home_sq.into();
            }
            MoveKind::Promotion(_) => {
                // Swap the piece type for the piece previously moved to to
                *moved_piece_bb ^= to.into();

                // Change moved piece over to pawn
                let pawn = Piece::new(us, PieceType::Pawn);
                *self.piece_mut(pawn) ^= to.into();
            }
            _ => (),
        }

        // Place captured piece back on square
        if let Some(piece) = state.captured_piece
            && mv.kind() != MoveKind::EnPassant
        {
            *self.piece_mut(piece) ^= from.into();
        }

        // Restore remaining board state
        self.castling_rights = state.castling_rights;
        self.half_move_clock = state.half_move_clock;
        self.en_passant_square = state.en_passant_square;
        self.side_to_move = us;
    }

    fn remove_rights_for_rook(&mut self, side: Color, rook_sq: Square) {
        let home_rank = if side.is_white() {
            Rank::One
        } else {
            Rank::Eight
        };
        let queen_side_rook_file = File::A;
        let king_side_rook_file = File::H;

        if rook_sq.rank() == home_rank
            && (rook_sq.file() == queen_side_rook_file || rook_sq.file() == king_side_rook_file)
        {
            let rights_downgrade = if rook_sq.file() == king_side_rook_file {
                CastlingRights::KingSide
            } else {
                CastlingRights::QueenSide
            };
            self.castling_rights[side] = self.castling_rights[side].downgrade(rights_downgrade);
        }
    }

    /// Returns whether or not a particular square is attacked by a specified side
    fn is_attacked(&self, target: Square, attacking_side: Color) -> bool {
        let pawns_bb = self.piece(Piece::new(attacking_side, PieceType::Pawn));
        if pawns_bb & pawn_attack_mask(!attacking_side, target) != Bitboard::EMPTY {
            return true;
        }

        let knights_bb = self.piece(Piece::new(attacking_side, PieceType::Knight));
        if knights_bb & attack_mask(PieceType::Knight, target) != Bitboard::EMPTY {
            return true;
        }

        let king_bb = self.piece(Piece::new(attacking_side, PieceType::King));
        if king_bb & attack_mask(PieceType::King, target) != Bitboard::EMPTY {
            return true;
        }

        let occupied = self.occupied();
        let bishop_or_queen_bb = self.piece(Piece::new(attacking_side, PieceType::Bishop))
            | self.piece(Piece::new(attacking_side, PieceType::Queen));
        if bishop_or_queen_bb & bishop_attacks(target, occupied) != Bitboard::EMPTY {
            return true;
        }

        let rook_or_queen_bb = self.piece(Piece::new(attacking_side, PieceType::Rook))
            | self.piece(Piece::new(attacking_side, PieceType::Queen));
        if rook_or_queen_bb & rook_attacks(target, occupied) != Bitboard::EMPTY {
            return true;
        }

        false
    }

    fn piece_mut(&mut self, piece: Piece) -> &mut Bitboard {
        &mut self.bitboards[piece]
    }

    fn board_state(&self) -> State {
        State {
            castling_rights: self.castling_rights,
            en_passant_square: self.en_passant_square,
            half_move_clock: self.half_move_clock,
            captured_piece: None,
        }
    }
}

// TODO: Add fullmove clock & impl Display for to FEN
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
            let (white_rights, black_rights) = rights_str.split_at(split_index);

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
        Self::from_str(STARTING_FEN).expect("failed to parse starting FEN position")
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
