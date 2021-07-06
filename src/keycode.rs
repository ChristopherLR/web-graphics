use crate::log;


macro_rules! console_log {
  ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[derive(Debug)]
pub enum KeyCode {
  A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
  Shift,
  Up,
  Down,
  Left,
  Right,
  Space,
  Other(u32)
}

use KeyCode::*;

impl KeyCode {
  pub fn from(key: u32) -> KeyCode {
    let kcode = match key {
      65 => A,
      66 => B,
      67 => C,
      68 => D,
      69 => E,
      70 => F,
      71 => G,
      72 => H,
      73 => I,
      74 => J,
      75 => K,
      76 => L,
      77 => M,
      78 => N,
      79 => O,
      80 => P,
      81 => Q,
      82 => R,
      83 => S,
      84 => T,
      85 => U,
      86 => V,
      87 => W,
      88 => X,
      89 => Y,
      90 => Z,
      16 => Shift,
      32 => Space,
      38 => Up,
      40 => Down,
      37 => Left,
      39 => Right,
      _ => Other(key)
    };

    // console_log!("{:?}", kcode);
    kcode
  }
}