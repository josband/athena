use lazy_static::{initialize, lazy_static};

use crate::chess::{
    Bitboard, CastlingRights, Color, Direction, File, NUM_COLORS, NUM_SQUARES, Piece, PieceType,
    Position, Rank, Square,
};

const NUM_ADJACENT_SQUARES: usize = 8;
const MAX_MOVES: usize = 256;

lazy_static! {
    static ref PAWN_ATTACK_MASKS: [[Bitboard; NUM_SQUARES]; NUM_COLORS] = gen_pawn_attack_masks();
    static ref KING_ATTACK_MASKS: [Bitboard; NUM_SQUARES] = gen_king_masks();
    static ref KNIGHT_ATTACK_MASKS: [Bitboard; NUM_SQUARES] = gen_knight_masks();
    static ref SLIDING_ATTACK_MASKS: [[Bitboard; NUM_SQUARES]; NUM_ADJACENT_SQUARES] =
        gen_sliding_masks();
}

/// Initializes global settings for move generation.
///
/// This is not required, but highly reccommended. If not, these settings will
/// be initialized on first access, which can hurt performance.
pub fn init_movegen() {
    initialize(&PAWN_ATTACK_MASKS);
    initialize(&KNIGHT_ATTACK_MASKS);
    initialize(&SLIDING_ATTACK_MASKS);
    initialize(&KING_ATTACK_MASKS);
}

/// Helper class for storing a list of moves for a particular position
///
/// This class contains a list of moves associated with a position. Moves are stored
/// in a pre-allocated array on the stack to help with performance.
pub struct MoveList {
    moves: [Option<Move>; MAX_MOVES],
    len: usize,
}

impl MoveList {
    pub fn new() -> Self {
        Self {
            moves: [None; MAX_MOVES],
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, index: usize) -> Option<Move> {
        if index < self.len() {
            self.moves[index]
        } else {
            None
        }
    }

    pub fn push(&mut self, mv: Move) {
        debug_assert!(self.len() < self.moves.len());
        self.moves[self.len()] = Some(mv);
        self.len += 1;
    }

    pub fn swap_remove(&mut self, index: usize) {
        debug_assert!(index < self.len);
        self.moves.swap(index, self.len - 1);

        self.moves[self.len() - 1] = None;
        self.len -= 1;
    }
}

impl Default for MoveList {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoIterator for MoveList {
    type Item = Move;

    type IntoIter = MoveListIter;

    fn into_iter(self) -> Self::IntoIter {
        MoveListIter {
            list: self,
            next: 0,
        }
    }
}

pub struct MoveListIter {
    list: MoveList,
    next: usize,
}

impl Iterator for MoveListIter {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next < self.list.len() {
            let result = self.list.moves[self.next].take();
            self.next += 1;
            result
        } else {
            None
        }
    }
}

pub(crate) fn attack_mask(piece_type: PieceType, square: Square) -> Bitboard {
    match piece_type {
        PieceType::Knight => KNIGHT_ATTACK_MASKS[square.lsf_index()],
        PieceType::Bishop => diagonal_masks(square),
        PieceType::Rook => orthogonal_masks(square),
        PieceType::Queen => diagonal_masks(square) | orthogonal_masks(square),
        PieceType::King => KING_ATTACK_MASKS[square.lsf_index()],
        _ => Bitboard::EMPTY,
    }
}

pub(crate) fn pawn_attack_mask(color: Color, square: Square) -> Bitboard {
    PAWN_ATTACK_MASKS[color as usize][square.lsf_index()]
}

fn diagonal_masks(square: Square) -> Bitboard {
    let square_index = square.lsf_index();
    SLIDING_ATTACK_MASKS[Direction::NorthEast.mask_cache_index()][square_index]
        | SLIDING_ATTACK_MASKS[Direction::NorthWest.mask_cache_index()][square_index]
        | SLIDING_ATTACK_MASKS[Direction::SouthEast.mask_cache_index()][square_index]
        | SLIDING_ATTACK_MASKS[Direction::SouthWest.mask_cache_index()][square_index]
}

fn orthogonal_masks(square: Square) -> Bitboard {
    let square_index = square.lsf_index();
    SLIDING_ATTACK_MASKS[Direction::North.mask_cache_index()][square_index]
        | SLIDING_ATTACK_MASKS[Direction::South.mask_cache_index()][square_index]
        | SLIDING_ATTACK_MASKS[Direction::East.mask_cache_index()][square_index]
        | SLIDING_ATTACK_MASKS[Direction::West.mask_cache_index()][square_index]
}

fn gen_pawn_attack_masks() -> [[Bitboard; NUM_SQUARES]; NUM_COLORS] {
    let mut pawn_masks = [[Bitboard::EMPTY; NUM_SQUARES]; NUM_COLORS];
    for square in Square::values() {
        if square.rank() == Rank::Eight {
            continue;
        }

        let sq_bb = Bitboard::from(square);
        pawn_masks[Color::White as usize][square.lsf_index()] =
            (sq_bb.shift(Direction::East) | sq_bb.shift(Direction::West)).shift(Direction::North);
    }

    for square in Square::values() {
        if square.rank() == Rank::One {
            continue;
        }

        let sq_bb = Bitboard::from(square);
        pawn_masks[Color::Black as usize][square.lsf_index()] =
            (sq_bb.shift(Direction::East) | sq_bb.shift(Direction::West)).shift(Direction::South);
    }

    pawn_masks
}

fn gen_king_masks() -> [Bitboard; NUM_SQUARES] {
    let mut king_masks = [Bitboard::EMPTY; NUM_SQUARES];
    for s in Square::values() {
        king_masks[s.lsf_index()] = king_attack_mask(s);
    }

    king_masks
}

fn king_attack_mask(square: Square) -> Bitboard {
    let king_bb = Bitboard::from(square);
    let attacks = king_bb.shift(Direction::East) | king_bb.shift(Direction::West);
    let full_row = king_bb | attacks;

    attacks | full_row.shift(Direction::North) | full_row.shift(Direction::South)
}

fn gen_sliding_masks() -> [[Bitboard; NUM_SQUARES]; NUM_ADJACENT_SQUARES] {
    let mut sliding_masks = [[Bitboard::EMPTY; NUM_SQUARES]; NUM_ADJACENT_SQUARES];
    for square in Square::values() {
        let square_bb = Bitboard::from(square);
        let [mut ne, mut nw, mut se, mut sw] = [square_bb; 4];
        let [mut n, mut e, mut s, mut w] = [square_bb; 4];
        for _ in 0..8 {
            ne |= ne.shift(Direction::NorthEast) & !Bitboard::from(File::A);
            nw |= nw.shift(Direction::NorthWest) & !Bitboard::from(File::H);
            se |= se.shift(Direction::SouthEast) & !Bitboard::from(File::A);
            sw |= sw.shift(Direction::SouthWest) & !Bitboard::from(File::H);
            n |= n.shift(Direction::North);
            e |= e.shift(Direction::East) & !Bitboard::from(File::A);
            s |= s.shift(Direction::South);
            w |= w.shift(Direction::West) & !Bitboard::from(File::H);
        }

        sliding_masks[Direction::North.mask_cache_index()][square.lsf_index()] = n & !square_bb;
        sliding_masks[Direction::South.mask_cache_index()][square.lsf_index()] = s & !square_bb;
        sliding_masks[Direction::East.mask_cache_index()][square.lsf_index()] = e & !square_bb;
        sliding_masks[Direction::West.mask_cache_index()][square.lsf_index()] = w & !square_bb;
        sliding_masks[Direction::NorthEast.mask_cache_index()][square.lsf_index()] =
            ne & !square_bb;
        sliding_masks[Direction::NorthWest.mask_cache_index()][square.lsf_index()] =
            nw & !square_bb;
        sliding_masks[Direction::SouthEast.mask_cache_index()][square.lsf_index()] =
            se & !square_bb;
        sliding_masks[Direction::SouthWest.mask_cache_index()][square.lsf_index()] =
            sw & !square_bb;
    }

    sliding_masks
}

fn gen_knight_masks() -> [Bitboard; NUM_SQUARES] {
    let mut knight_moves = [Bitboard::EMPTY; NUM_SQUARES];
    for s in Square::values() {
        let origin = Bitboard::from(s);

        // Compute vertical-major moves
        let nne =
            origin.shift_n(Direction::North, 2).shift(Direction::East) & !Bitboard::from(File::A);
        let nnw =
            origin.shift_n(Direction::North, 2).shift(Direction::West) & !Bitboard::from(File::H);
        let sse =
            origin.shift_n(Direction::South, 2).shift(Direction::East) & !Bitboard::from(File::A);
        let ssw =
            origin.shift_n(Direction::South, 2).shift(Direction::West) & !Bitboard::from(File::H);

        // Compute horizontal-major moves
        let nee = origin.shift(Direction::North).shift_n(Direction::East, 2)
            & !(Bitboard::from(File::A) | Bitboard::from(File::B));
        let see = origin.shift(Direction::South).shift_n(Direction::East, 2)
            & !(Bitboard::from(File::A) | Bitboard::from(File::B));
        let nww = origin.shift(Direction::North).shift_n(Direction::West, 2)
            & !(Bitboard::from(File::G) | Bitboard::from(File::H));
        let sww = origin.shift(Direction::South).shift_n(Direction::West, 2)
            & !(Bitboard::from(File::G) | Bitboard::from(File::H));

        knight_moves[s.lsf_index()] = nne | nnw | sse | ssw | nee | see | nww | sww;
    }

    knight_moves
}

impl Direction {
    fn mask_cache_index(&self) -> usize {
        match self {
            Direction::North => 0,
            Direction::South => 1,
            Direction::East => 2,
            Direction::West => 3,
            Direction::NorthEast => 4,
            Direction::NorthWest => 5,
            Direction::SouthEast => 6,
            Direction::SouthWest => 7,
        }
    }

    fn is_negative(&self) -> bool {
        matches!(
            self,
            Direction::South | Direction::SouthEast | Direction::SouthWest | Direction::West
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MoveKind {
    Quiet,
    Capture,
    EnPassant,
    Castle,
    Promotion(PieceType),
}

/// A piece movement in chess.
///
/// In chess, a move commonly refers to a piece movement from **both**
/// sides. In chess programming, a move represents a single piece movement, which is
/// otherwise called a half-move or a ply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Move {
    from: Square,
    to: Square,
    kind: MoveKind,
}

impl Move {
    pub fn new(from: Square, to: Square, kind: MoveKind) -> Self {
        Self { from, to, kind }
    }

    pub fn from_sq(&self) -> Square {
        self.from
    }

    pub fn to_sq(&self) -> Square {
        self.to
    }

    pub fn kind(&self) -> MoveKind {
        self.kind
    }

    pub fn to_uci_string(&self) -> String {
        let promotion_char = if let MoveKind::Promotion(promotion_piece_type) = self.kind() {
            &promotion_piece_type.to_string()
        } else {
            ""
        };

        format!("{}{}{}", self.from_sq(), self.to_sq(), promotion_char)
    }
}

/// Generates all pseudo-legal moves for a given position
pub fn generate_moves(position: &Position, moves: &mut MoveList) {
    pawn_moves(position, moves);
    knight_moves(position, moves);
    bishop_moves(position, moves);
    rook_moves(position, moves);
    queen_moves(position, moves);
    king_moves(position, moves);
}

fn pawn_moves(position: &Position, moves: &mut MoveList) {
    let side = position.side_to_move();
    let (forward, forward_left, forward_right) = if side.is_white() {
        (Direction::North, Direction::NorthWest, Direction::NorthEast)
    } else {
        (Direction::South, Direction::SouthEast, Direction::SouthWest)
    };

    let double_push_rank = if side.is_white() {
        Bitboard::from(Rank::Four)
    } else {
        Bitboard::from(Rank::Five)
    };

    let promotion_rank = if side.is_white() {
        Bitboard::from(Rank::Seven)
    } else {
        Bitboard::from(Rank::Two)
    };

    let pawns = position.piece(Piece::new(side, PieceType::Pawn));
    let promotion_eligible_pawns = pawns & promotion_rank;
    let promotion_ineligible_pawns = pawns & !promotion_rank;
    let enemy_pieces = position.color_pieces(!side);
    let empty = position.empty_squares();

    // Compute pawn pushes (exclude promotions)
    let single_push = promotion_ineligible_pawns.shift(forward) & empty;
    let double_push = single_push.shift(forward) & empty & double_push_rank;

    insert_pawn_moves(moves, single_push, forward as i32);
    insert_pawn_moves(moves, double_push, forward + forward);

    // Compute pawn captures
    let captures_right = promotion_ineligible_pawns.shift(forward_right) & enemy_pieces;
    let captures_left = promotion_ineligible_pawns.shift(forward_left) & enemy_pieces;
    insert_pawn_captures(moves, captures_left, forward_left as i32);
    insert_pawn_captures(moves, captures_right, forward_right as i32);

    if position.has_en_passant() {
        let mut en_passant_capture_right = promotion_ineligible_pawns.shift(forward_right)
            & position.en_passant_square().unwrap().into();
        while let Some(to) = en_passant_capture_right.pop_lsb() {
            let from = Bitboard::from(to)
                .shift(forward_right.flip())
                .pop_lsb()
                .unwrap();
            moves.push(Move::new(from, to, MoveKind::EnPassant));
        }

        let mut en_passant_capture_left = promotion_ineligible_pawns.shift(forward_left)
            & position.en_passant_square().unwrap().into();
        while let Some(to) = en_passant_capture_left.pop_lsb() {
            let from = Bitboard::from(to)
                .shift(forward_left.flip())
                .pop_lsb()
                .unwrap();
            moves.push(Move::new(from, to, MoveKind::EnPassant));
        }
    }

    // Compute promotions
    let push_promotions = promotion_eligible_pawns.shift(forward) & empty;
    let capture_promo_right = promotion_eligible_pawns.shift(forward_right) & enemy_pieces;
    let capture_promo_left = promotion_eligible_pawns.shift(forward_left) & enemy_pieces;

    insert_promotion(moves, push_promotions, forward);
    insert_promotion(moves, capture_promo_right, forward_right);
    insert_promotion(moves, capture_promo_left, forward_left);
}

fn insert_pawn_captures(moves: &mut MoveList, mut to_bb: Bitboard, direction: i32) {
    while let Some(to) = to_bb.pop_lsb() {
        moves.push(Move::new((to - direction).unwrap(), to, MoveKind::Capture));
    }
}

fn insert_pawn_moves(moves: &mut MoveList, mut to_bb: Bitboard, direction: i32) {
    while let Some(to) = to_bb.pop_lsb() {
        moves.push(Move::new((to - direction).unwrap(), to, MoveKind::Quiet));
    }
}

fn insert_promotion(moves: &mut MoveList, mut dest_bb: Bitboard, direction: Direction) {
    while let Some(dest) = dest_bb.pop_lsb() {
        make_promotion(moves, (dest - direction as i32).unwrap(), dest);
    }
}

fn make_promotion(moves: &mut MoveList, source: Square, dest: Square) {
    moves.push(Move::new(
        source,
        dest,
        MoveKind::Promotion(PieceType::Knight),
    ));
    moves.push(Move::new(
        source,
        dest,
        MoveKind::Promotion(PieceType::Bishop),
    ));
    moves.push(Move::new(
        source,
        dest,
        MoveKind::Promotion(PieceType::Rook),
    ));
    moves.push(Move::new(
        source,
        dest,
        MoveKind::Promotion(PieceType::Queen),
    ));
}

fn knight_moves(position: &Position, moves: &mut MoveList) {
    let blockers = position.color_pieces(position.side_to_move());
    let mut knights = position.piece(Piece::new(position.side_to_move(), PieceType::Knight));
    while let Some(source) = knights.pop_lsb() {
        let hops = KNIGHT_ATTACK_MASKS[source.lsf_index()] & !blockers;
        insert_moves(position, source, hops, moves);
    }
}

fn bishop_moves(position: &Position, moves: &mut MoveList) {
    let to_move = position.side_to_move();
    let occupied = position.occupied();
    let friendly = position.color_pieces(to_move);
    let mut bishops = position.piece(Piece::new(to_move, PieceType::Bishop));

    while let Some(source) = bishops.pop_lsb() {
        let bishop_moves = bishop_attacks(source, occupied) & !friendly;
        insert_moves(position, source, bishop_moves, moves);
    }
}

fn rook_moves(position: &Position, moves: &mut MoveList) {
    let to_move = position.side_to_move();
    let occupied = position.occupied();
    let friendly = position.color_pieces(to_move);
    let mut rooks = position.piece(Piece::new(to_move, PieceType::Rook));

    while let Some(source) = rooks.pop_lsb() {
        let rook_moves = rook_attacks(source, occupied) & !friendly;
        insert_moves(position, source, rook_moves, moves);
    }
}

fn queen_moves(position: &Position, moves: &mut MoveList) {
    let to_move = position.side_to_move();
    let occupied = position.occupied();
    let friendly = position.color_pieces(to_move);
    let mut queens = position.piece(Piece::new(to_move, PieceType::Queen));

    while let Some(source) = queens.pop_lsb() {
        let queen_moves = queen_attacks(source, occupied) & !friendly;
        insert_moves(position, source, queen_moves, moves);
    }
}

fn king_moves(position: &Position, moves: &mut MoveList) {
    let to_move = position.side_to_move();
    let castle_mask_shift_value = if to_move.is_white() { 0 } else { 56 };
    let queen_castle_mask = Bitboard(0xe << castle_mask_shift_value);
    let king_castle_mask = Bitboard(0x60 << castle_mask_shift_value);
    let castle_mask = king_castle_mask | queen_castle_mask;
    let king = position.piece(Piece::new(to_move, PieceType::King));
    let king_square = king.lsb().unwrap();
    let free = position.empty_squares();
    let mut captures_bb =
        KING_ATTACK_MASKS[king_square.lsf_index()] & position.color_pieces(!to_move);
    let mut quiet_bb = KING_ATTACK_MASKS[king_square.lsf_index()] & free;
    let castle_bb = castle_mask & free;

    while let Some(capture_square) = captures_bb.pop_lsb() {
        moves.push(Move::new(king_square, capture_square, MoveKind::Capture));
    }

    while let Some(move_square) = quiet_bb.pop_lsb() {
        moves.push(Move::new(king_square, move_square, MoveKind::Quiet));
    }

    match position.castling_rights(to_move) {
        CastlingRights::All => {
            generate_king_castle(king_square, castle_bb, king_castle_mask, moves);
            generate_queen_castle(king_square, castle_bb, queen_castle_mask, moves);
        }
        CastlingRights::KingSide => {
            generate_king_castle(king_square, castle_bb, king_castle_mask, moves);
        }
        CastlingRights::QueenSide => {
            generate_queen_castle(king_square, castle_bb, queen_castle_mask, moves);
        }
        _ => (),
    }
}

fn insert_moves(
    position: &Position,
    source: Square,
    mut locations: Bitboard,
    moves: &mut MoveList,
) {
    while let Some(dest) = locations.pop_lsb() {
        let dest_occupancy_opt = position.get_piece_at(&dest);
        if dest_occupancy_opt.is_some_and(|p| p.color() != position.side_to_move()) {
            moves.push(Move::new(source, dest, MoveKind::Capture));
        } else if dest_occupancy_opt.is_none() {
            moves.push(Move::new(source, dest, MoveKind::Quiet));
        };
    }
}

fn generate_queen_castle(
    king_square: Square,
    open_castle_bb: Bitboard,
    castle_mask: Bitboard,
    moves: &mut MoveList,
) {
    if open_castle_bb & castle_mask == castle_mask {
        let castle_square = Square::new(king_square.file().left_n(2).unwrap(), king_square.rank());
        moves.push(Move::new(king_square, castle_square, MoveKind::Castle));
    }
}

fn generate_king_castle(
    king_square: Square,
    open_castle_bb: Bitboard,
    castle_mask: Bitboard,
    moves: &mut MoveList,
) {
    if open_castle_bb & castle_mask == castle_mask {
        let castle_square = Square::new(king_square.file().right_n(2).unwrap(), king_square.rank());
        moves.push(Move::new(king_square, castle_square, MoveKind::Castle));
    }
}

pub(crate) fn queen_attacks(square: Square, occupied: Bitboard) -> Bitboard {
    rook_attacks(square, occupied) | bishop_attacks(square, occupied)
}

pub(crate) fn rook_attacks(square: Square, occupied: Bitboard) -> Bitboard {
    let file_attacks = get_ray_attacks(square, Direction::North, occupied)
        | get_ray_attacks(square, Direction::South, occupied);
    let rank_attacks = get_ray_attacks(square, Direction::East, occupied)
        | get_ray_attacks(square, Direction::West, occupied);

    file_attacks | rank_attacks
}

pub(crate) fn bishop_attacks(square: Square, occupied: Bitboard) -> Bitboard {
    let diagonal = get_ray_attacks(square, Direction::NorthEast, occupied)
        | get_ray_attacks(square, Direction::SouthWest, occupied);
    let anti_diagonal = get_ray_attacks(square, Direction::NorthWest, occupied)
        | get_ray_attacks(square, Direction::SouthEast, occupied);

    diagonal | anti_diagonal
}

fn get_ray_attacks(square: Square, dir: Direction, occupied: Bitboard) -> Bitboard {
    let mut attacks = SLIDING_ATTACK_MASKS[dir.mask_cache_index()][square.lsf_index()];
    let blockers = attacks & occupied;
    let blocker = if dir.is_negative() {
        blockers.msb()
    } else {
        blockers.lsb()
    };

    if let Some(block_square) = blocker {
        attacks ^= SLIDING_ATTACK_MASKS[dir.mask_cache_index()][block_square.lsf_index()];
    }

    attacks
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::chess::{Error, State};

    use super::*;

    const START_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    const KIWIPETE: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
    const POSITION_3: &str = "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1";
    const POSITION_4: &str = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";
    const POSITION_5: &str = "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8";
    const POSITION_6: &str =
        "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 ";

    #[test]
    fn test_start_position_perft() {
        let result = do_perft(START_POSITION, 6, 119060324);

        assert!(result.is_ok());
    }

    #[test]
    fn test_kiwipete_perft() {
        let result = do_perft(KIWIPETE, 5, 193690690);

        assert!(result.is_ok());
    }

    #[test]
    fn test_position_3_perft() {
        let result = do_perft(POSITION_3, 7, 178633661);

        assert!(result.is_ok());
    }

    #[test]
    fn test_position_4_perft() {
        let result = do_perft(POSITION_4, 5, 15833292);

        assert!(result.is_ok());
    }

    #[test]
    fn test_position_5_perft() {
        let result = do_perft(POSITION_5, 5, 89941194);

        assert!(result.is_ok());
    }

    #[test]
    fn test_position_6_perft() {
        let result = do_perft(POSITION_6, 5, 164075551);

        assert!(result.is_ok());
    }

    /// Generates all legal moves for a given position
    fn generate_legal_moves(position: &mut Position, moves: &mut MoveList) {
        let mut curr = moves.len();
        generate_moves(position, moves);

        let mut history = vec![];
        while curr < moves.len() {
            let mv = moves.get(curr).expect("move does not exist");
            if !position.make_move(mv, &mut history) {
                moves.swap_remove(curr);
            } else {
                curr += 1;
                position.unmake_move(mv, &mut history);
            }
        }
    }

    /// Perft testing function
    fn perft(pos: &mut Position, history: &mut Vec<State>, depth: u64) -> u64 {
        if depth == 0 {
            return 1;
        }

        let mut moves = MoveList::new();
        generate_legal_moves(pos, &mut moves);

        if depth == 1 {
            return moves.len() as u64;
        }

        let mut move_count = 0;
        for mv in moves {
            if !pos.make_move(mv, history) {
                panic!("legal move couldn't be made");
            }
            move_count += perft(pos, history, depth - 1);
            pos.unmake_move(mv, history);
        }

        move_count
    }

    fn perft_divide(pos: &mut Position, history: &mut Vec<State>, depth: u64) -> u64 {
        if depth == 0 {
            return 1;
        }

        let mut moves = MoveList::new();
        generate_legal_moves(pos, &mut moves);

        let mut move_count = 0;
        for mv in moves {
            if !pos.make_move(mv, history) {
                panic!("legal move couldn't be made");
            }
            let subtree_count = if depth == 1 {
                1
            } else {
                perft(pos, history, depth - 1)
            };
            move_count += subtree_count;
            pos.unmake_move(mv, history);
            println!("{}: {}", mv.to_uci_string(), subtree_count);
        }

        println!("\nNodes searched: {}", move_count);
        move_count
    }

    fn do_perft(pos: &str, starting_depth: u64, expected_count: u64) -> Result<(), Error> {
        let mut position = Position::from_str(pos)?;
        let mut history = Vec::new();
        let legal_count = perft_divide(&mut position, &mut history, starting_depth);

        assert_eq!(expected_count, legal_count);

        Ok(())
    }
}
