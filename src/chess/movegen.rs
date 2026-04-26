use lazy_static::{initialize, lazy_static};

use crate::chess::{
    Bitboard, CastlingRights, Direction, File, NUM_SQUARES, Piece, PieceType, Position, Rank,
    Square,
};

const NUM_ADJACENT_SQUARES: usize = 8;

lazy_static! {
    static ref KNIGHT_ATTACK_MASKS: [Bitboard; NUM_SQUARES] = knight_attack_masks();
    static ref SLIDING_ATTACK_MASKS: [[Bitboard; NUM_SQUARES]; NUM_ADJACENT_SQUARES] =
        sliding_attack_masks();
    static ref KING_ATTACK_MASKS: [Bitboard; NUM_SQUARES] = king_attack_masks();
}

/// Initializes global settings for move generation.
///
/// This is not required, but highly reccommended. If not, these settings will
/// be initialized on first access, which can hurt performance.
pub fn init_movegen() {
    initialize(&KNIGHT_ATTACK_MASKS);
    initialize(&SLIDING_ATTACK_MASKS);
    initialize(&KING_ATTACK_MASKS);
}

fn king_attack_masks() -> [Bitboard; NUM_SQUARES] {
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

fn sliding_attack_masks() -> [[Bitboard; NUM_SQUARES]; NUM_ADJACENT_SQUARES] {
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

fn knight_attack_masks() -> [Bitboard; NUM_SQUARES] {
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
}

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// struct MoveBuilder {
//     from: Option<Square>,
//     to: Option<Square>,
//     promotion_piece: Option<PieceType>,
// }

// impl MoveBuilder {
//     pub fn new() -> Self {
//         Self { from: None, to: None, promotion_piece: None }
//     }

//     pub fn build(self) -> Move {
//         todo!()
//     }
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Move {
    source: Square,
    target: Square,
    promotion_piece: Option<PieceType>,
    captured_piece: Option<PieceType>,
    // is_castle: bool,
    // is_ep: bool,
}

impl Move {
    pub fn new(
        source: Square,
        target: Square,
        promotion_piece: Option<PieceType>,
        captured_piece: Option<PieceType>,
    ) -> Self {
        Self {
            source,
            target,
            promotion_piece,
            captured_piece,
        }
    }

    pub fn from_sq(&self) -> Square {
        self.source
    }

    pub fn to_sq(&self) -> Square {
        self.target
    }

    // TODO: Need to track the special move
    pub fn is_castle_move(&self) -> bool {
        false
    }

    // TODO: Need to track the special move
    pub fn is_en_passant_move(&self) -> bool {
        false
    }

    pub fn is_promotion(&self) -> bool {
        self.promotion_piece.is_some()
    }

    pub fn is_capture(&self) -> bool {
        self.captured_piece.is_some()
    }
}

/// Generates all pseudo-legal moves for a given position
pub fn generate_moves(position: &Position) -> Vec<Move> {
    let mut moves = vec![];
    pawn_moves(position, &mut moves);
    knight_moves(position, &mut moves);
    bishop_moves(position, &mut moves);
    rook_moves(position, &mut moves);
    queen_moves(position, &mut moves);
    king_moves(position, &mut moves);

    moves
}

// TODO: Clean up, also missing capture promotions
fn pawn_moves(position: &Position, moves: &mut Vec<Move>) {
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
        Rank::Eight
    } else {
        Rank::One
    };

    let mut pawns = position.piece(Piece::new(side, PieceType::Pawn));
    let enemy_pieces = position.color_pieces(!side);
    let empty = position.empty_squares();

    // Compute pawn pushes
    let single_push = pawns.shift(forward) & empty;
    let double_push = single_push.shift(forward) & empty & double_push_rank;
    let pushes = single_push | double_push;

    // Compute pawn captures, including en passant
    let en_passant_target = position
        .en_passant_square()
        .map(Bitboard::from)
        .unwrap_or(Bitboard::EMPTY);
    let left_captures = pawns.shift(forward_left) & enemy_pieces;
    let right_captures = pawns.shift(forward_right) & enemy_pieces;
    let captures = left_captures | right_captures;

    while let Some(source) = pawns.pop_lsb() {
        let pawn_bb: Bitboard = source.into();

        let push_file_bb: Bitboard = source.file().into();
        let mut push_targets = push_file_bb & pushes;
        while let Some(destination) = push_targets.pop_lsb() {
            if destination.rank() == promotion_rank {
                moves.push(Move::new(
                    source,
                    destination,
                    Some(PieceType::Knight),
                    None,
                ));
                moves.push(Move::new(
                    source,
                    destination,
                    Some(PieceType::Bishop),
                    None,
                ));
                moves.push(Move::new(source, destination, Some(PieceType::Rook), None));
                moves.push(Move::new(source, destination, Some(PieceType::Queen), None));
            } else {
                moves.push(Move::new(source, destination, None, None));
            }
        }

        let capture_targets =
            captures & (pawn_bb.shift(forward_left) | pawn_bb.shift(forward_right));
        insert_moves(position, source, capture_targets, moves);

        // TODO: This doesn't account for En passant from both sides
        let en_passant =
            en_passant_target & (pawn_bb.shift(forward_left) | pawn_bb.shift(forward_right));
        if en_passant != Bitboard::EMPTY {
            moves.push(Move::new(
                source,
                position.en_passant_square().unwrap(),
                None,
                position
                    .get_piece_at(position.en_passant_square().unwrap())
                    .map(|p| p.piece_type()),
            ));
        }
    }
}

fn knight_moves(position: &Position, moves: &mut Vec<Move>) {
    let blockers = position.color_pieces(position.side_to_move());
    let mut knights = position.piece(Piece::new(position.side_to_move(), PieceType::Knight));
    while let Some(source) = knights.pop_lsb() {
        let hops = KNIGHT_ATTACK_MASKS[source.lsf_index()] & !blockers;
        insert_moves(position, source, hops, moves);
    }
}

fn bishop_moves(position: &Position, moves: &mut Vec<Move>) {
    let to_move = position.side_to_move();
    let occupied = position.occupied();
    let friendly = position.color_pieces(to_move);
    let mut bishops = position.piece(Piece::new(to_move, PieceType::Bishop));

    while let Some(source) = bishops.pop_lsb() {
        let bishop_moves = bishop_attacks(source, occupied) & !friendly;
        insert_moves(position, source, bishop_moves, moves);
    }
}

fn rook_moves(position: &Position, moves: &mut Vec<Move>) {
    let to_move = position.side_to_move();
    let occupied = position.occupied();
    let friendly = position.color_pieces(to_move);
    let mut rooks = position.piece(Piece::new(to_move, PieceType::Rook));

    while let Some(source) = rooks.pop_lsb() {
        let rook_moves = rook_attacks(source, occupied) & !friendly;
        insert_moves(position, source, rook_moves, moves);
    }
}

fn queen_moves(position: &Position, moves: &mut Vec<Move>) {
    let to_move = position.side_to_move();
    let occupied = position.occupied();
    let friendly = position.color_pieces(to_move);
    let mut queens = position.piece(Piece::new(to_move, PieceType::Queen));

    while let Some(source) = queens.pop_lsb() {
        let queen_moves = queen_attacks(source, occupied) & !friendly;
        insert_moves(position, source, queen_moves, moves);
    }
}

fn king_moves(position: &Position, moves: &mut Vec<Move>) {
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
        moves.push(Move::new(
            king_square,
            capture_square,
            None,
            position
                .get_piece_at(capture_square)
                .map(|p| p.piece_type()),
        ));
    }

    while let Some(move_square) = quiet_bb.pop_lsb() {
        moves.push(Move::new(king_square, move_square, None, None));
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
    moves: &mut Vec<Move>,
) {
    while let Some(dest) = locations.pop_lsb() {
        moves.push(Move::new(
            source,
            dest,
            None,
            position.get_piece_at(dest).map(|p| p.piece_type()),
        ));
    }
}

fn generate_queen_castle(
    king_square: Square,
    open_castle_bb: Bitboard,
    castle_mask: Bitboard,
    moves: &mut Vec<Move>,
) {
    if open_castle_bb & castle_mask == castle_mask {
        let castle_square = Square::new(king_square.file().left_n(2).unwrap(), king_square.rank());
        moves.push(Move::new(king_square, castle_square, None, None));
    }
}

fn generate_king_castle(
    king_square: Square,
    open_castle_bb: Bitboard,
    castle_mask: Bitboard,
    moves: &mut Vec<Move>,
) {
    if open_castle_bb & castle_mask == castle_mask {
        let castle_square = Square::new(king_square.file().right_n(2).unwrap(), king_square.rank());
        moves.push(Move::new(king_square, castle_square, None, None));
    }
}

fn queen_attacks(square: Square, occupied: Bitboard) -> Bitboard {
    rook_attacks(square, occupied) | bishop_attacks(square, occupied)
}

fn rook_attacks(square: Square, occupied: Bitboard) -> Bitboard {
    let file_attacks = get_ray_attacks(square, Direction::North, occupied)
        | get_ray_attacks(square, Direction::South, occupied);
    let rank_attacks = get_ray_attacks(square, Direction::East, occupied)
        | get_ray_attacks(square, Direction::West, occupied);

    file_attacks | rank_attacks
}

fn bishop_attacks(square: Square, occupied: Bitboard) -> Bitboard {
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
    use super::*;

    #[test]
    fn test_name() {
        let p = Position::default();
        let moves = generate_moves(&p);

        assert_eq!(20, moves.len());
    }
}
