pub const NUM_COLOR: usize = 2;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[repr(u8)]
pub enum COLOR {
  WHITE,
  BLACK,
}
