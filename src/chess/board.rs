use super::bitboard::Bitboard;
use super::types::{NUM_COLOR, NUM_PIECE};

pub struct Board {
  piece_bitboards: [Bitboard; NUM_PIECE],
  color_bitboards: [Bitboard; NUM_COLOR],
  hash: u64,
}
