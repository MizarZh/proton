use super::bitboard::Bitboard;
use super::pieces::NUM_PIECE;
use super::colors::NUM_COLOR;


pub struct Board {
  piece_bitboards: [Bitboard; NUM_PIECE],
  color_bitboards: [Bitboard; NUM_COLOR],
  hash: u64,
}
