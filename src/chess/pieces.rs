pub const NUM_PIECE: usize = 14;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(u8)]
pub enum Pieces {
  WP,
  WN,
  WB,
  WR,
  WQ,
  WK,
  BP,
  BN,
  BB,
  BR,
  BQ,
  BK,
}

pub const NUM_PIECE_TYPE: usize = 6;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(u8)]
pub enum PieceType {
  PAWN,
  KNIGHT,
  BISHOP,
  ROOK,
  QUEEN,
  KING,
}