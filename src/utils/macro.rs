macro_rules! simple_enum {
  (
      $(#[$attr:meta])*
      $vis:vis enum $name:ident {
          $(
              $(#[$variant_attr:meta])*
              $variant:ident
          ),*
      }
  ) => {
      $(#[$attr])*
      $vis enum $name {
          $(
              $(#[$variant_attr])*
              $variant
          ),*
      }

      impl $name {
          #[doc = concat!("The number of [`", stringify!($name), "`] variants.")]
          pub const NUM: usize = [$(Self::$variant),*].len();
          #[doc = concat!("An array of all [`", stringify!($name), "`] variants.")]
          pub const ALL: [Self; Self::NUM] = [$(Self::$variant),*];

          #[doc = concat!("Checked version of [`", stringify!($name), "::index`].")]
          #[inline(always)]
          pub const fn try_index(index: usize) -> Option<Self> {
              mod variant_indexes {
                  #![allow(non_upper_case_globals, unused)]
                  $(pub const $variant: usize = super::$name::$variant as usize;)*
              }
              #[allow(non_upper_case_globals)]
              match index {
                  $(variant_indexes::$variant => Some(Self::$variant),)*
                  _ => None
              }
          }

          #[doc = concat!(
              "Convert an index to a [`", stringify!($name), "`].\n",
              "# Panics\n",
              "Panic if the index is out of bounds."
          )]
          #[inline(always)]
          pub fn index(index: usize) -> Self {
              Self::try_index(index).unwrap_or_else(|| panic!("Index {} is out of range.", index))
          }

          #[doc = concat!(
              "`const` version of [`", stringify!($name), "::index`].\n",
              "# Panics\n",
              "Panic if the index is out of bounds."
          )]
          #[inline(always)]
          pub const fn index_const(index: usize) -> Self {
              if let Some(value) = Self::try_index(index) {
                  value
              } else {
                  panic!("Index is out of range")
              }
          }
      }
  };
}
pub(crate) use simple_enum;
