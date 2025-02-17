use super::squares::{NUM_FILE, NUM_RANK, TOTAL_SQUARE, NUM_SQUARE};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Bitboard(pub u64);

impl Bitboard {
  fn get_lsb(bb: Bitboard) -> u32 {
    assert_eq!(bb.0, 0);
    bb.0.trailing_zeros()
  }

  fn get_msb(bb: Bitboard) -> u32 {
    assert_eq!(bb.0, 0);
    bb.0.leading_zeros()
  }

  fn pop_lsb(bb: &mut Bitboard) -> u32 {
    let lsb = Bitboard::get_lsb(*bb);
    (*bb).0 &= (*bb).0 - 1;
    lsb
  }

  fn pop_msb(bb: &mut Bitboard) -> u32 {
    let msb = Bitboard::get_msb(*bb);
    (*bb).0 ^= 1u64 << msb;
    msb
  }

  fn test_bit(bb: Bitboard, i: usize) -> bool {
    assert!(i < NUM_SQUARE);
    bb.0 & (1u64 << i) != 0
  }

  fn set_bit(bb: &mut Bitboard, i: usize) {
    assert!(Bitboard::test_bit(*bb, i));
    (*bb).0 ^= 1u64 << i;
  }

  fn clear_bit(bb: &mut Bitboard, i: usize) {
    assert!(!Bitboard::test_bit(*bb, i));
    (*bb).0 ^= 1u64 << i;
  }

  fn several(bb: Bitboard) -> bool {
    bb.0 & (bb.0 - 1) != 0
  }

  fn onlyOne(bb: Bitboard) -> bool {
    bb.0 != 0 && !Bitboard::several(bb)
  }
}

macro_rules! bitboard_array {
  ($($val:expr),* $(,)?) => {
      [$(Bitboard($val)),*]  // Expands into a fixed-size array
  };
}

pub const FILE_MASK: [Bitboard; NUM_FILE] = bitboard_array![
  0x101010101010101,
  0x202020202020202,
  0x404040404040404,
  0x808080808080808,
  0x1010101010101010,
  0x2020202020202020,
  0x4040404040404040,
  0x8080808080808080,
];

pub const RANK_MASK: [Bitboard; NUM_RANK] = bitboard_array![
  0xff,
  0xff00,
  0xff0000,
  0xff000000,
  0xff00000000,
  0xff0000000000,
  0xff000000000000,
  0xff00000000000000,
];

#[rustfmt::skip]
pub const SQUARE_MASK: [Bitboard; TOTAL_SQUARE] = bitboard_array![
  0x1, 0x2, 0x4, 0x8,
  0x10, 0x20, 0x40, 0x80,
  0x100, 0x200, 0x400, 0x800,
  0x1000, 0x2000, 0x4000, 0x8000,
  0x10000, 0x20000, 0x40000, 0x80000,
  0x100000, 0x200000, 0x400000, 0x800000,
  0x1000000, 0x2000000, 0x4000000, 0x8000000,
  0x10000000, 0x20000000, 0x40000000, 0x80000000,
  0x100000000, 0x200000000, 0x400000000, 0x800000000,
  0x1000000000, 0x2000000000, 0x4000000000, 0x8000000000,
  0x10000000000, 0x20000000000, 0x40000000000, 0x80000000000,
  0x100000000000, 0x200000000000, 0x400000000000, 0x800000000000,
  0x1000000000000, 0x2000000000000, 0x4000000000000, 0x8000000000000,
  0x10000000000000, 0x20000000000000, 0x40000000000000, 0x80000000000000,
  0x100000000000000, 0x200000000000000, 0x400000000000000, 0x800000000000000,
  0x1000000000000000, 0x2000000000000000, 0x4000000000000000, 0x8000000000000000,
  0x0
];

pub const WHITE_SQUIRE_MASK: Bitboard = Bitboard(0x55AA55AA55AA55AA);
pub const BLACK_SQUARE_MASK: Bitboard = Bitboard(0xAA55AA55AA55AA55);
