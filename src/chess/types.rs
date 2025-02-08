

pub const NUM_PIECE: usize = 14;
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(u8)]
pub enum PIECE {
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
pub enum PieceType {
  PAWN,
  KNIGHT,
  BISHOP,
  ROOK,
  QUEEN,
  KING,
}

pub const NUM_SQUARE: usize = 64;
#[rustfmt::skip]
pub enum SQUARE {
  A1, B1, C1, D1, E1, F1, G1, H1,
  A2, B2, C2, D2, E2, F2, G2, H2,
  A3, B3, C3, D3, E3, F3, G3, H3,
  A4, B4, C4, D4, E4, F4, G4, H4,
  A5, B5, C5, D5, E5, F5, G5, H5,
  A6, B6, C6, D6, E6, F6, G6, H6,
  A7, B7, C7, D7, E7, F7, G7, H7,
  A8, B8, C8, D8, E8, F8, G8, H8,
  NO_SQUARE,
}

pub const NUM_FILE: usize = 8;
pub enum FILE {
  AFILE,
  BFILE,
  CFILE,
  DFILE,
  EFILE,
  FFILE,
  GFILE,
  HFILE,
}

pub const NUM_RANK: usize = 8;
pub enum RANK {
  RANK1,
  RANK2,
  RANK3,
  RANK4,
  RANK5,
  RANK6,
  RANK7,
  RANK8,
}

pub const NUM_COLOR: usize = 2;
pub enum COLOR {
  WHITE,
  BLACK,
}
