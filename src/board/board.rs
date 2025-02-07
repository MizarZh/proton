use types::{NUM_PIECE, NUM_COLOR};
use bitbaord::Bitboard;

pub struct Board {
  piece_bitboards: [Bitboard; NUM_PIECE],
  color_bitboards: [Bitboard; NUM_COLOR]
}