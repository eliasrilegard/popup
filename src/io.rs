//! Kattis IO.

use std::io::Read;
use std::str::{FromStr, from_utf8_unchecked};

/// A fast input reader that reads from stdin and allows parsing of tokens and
/// lines on demand.
#[doc(alias = "Reader")]
pub struct Gimme {
  buf: Vec<u8>,
  pos: usize,
}

impl Default for Gimme {
  fn default() -> Self {
    Self::new()
  }
}

// Whitespace is skipped when beginning an operation, but never implicitly
// consumed beyond what is necessary to establish the operation's result.
impl Gimme {
  /// Create a new `Reader` that reads and stores all input from stdin.
  pub fn new() -> Self {
    let mut buf = Vec::new();
    std::io::stdin().read_to_end(&mut buf).unwrap();

    Self { buf, pos: 0 }
  }

  /// Return whether there are more tokens to read from the input.
  pub fn has_more(&mut self) -> bool {
    self.skip_whitespace();
    self.pos < self.buf.len()
  }

  /// Return the next token parsed as type `T`. Panic if input is
  /// exhausted or the token cannot be parsed as `T`.
  #[inline]
  #[allow(clippy::should_implement_trait)]
  pub fn next<T>(&mut self) -> T
  where
    T: FromStr,
    T::Err: std::fmt::Debug,
  {
    self.next_opt().expect("Input exhausted")
  }

  /// Return the next token parsed as type `T`, or `None` if input is
  /// exhausted. Panic if the token cannot be parsed as `T`.
  #[inline]
  pub fn next_opt<T>(&mut self) -> Option<T>
  where
    T: FromStr,
    T::Err: std::fmt::Debug,
  {
    self
      .token()
      .map(|s| s.parse().expect("Failed to parse token"))
  }

  /// Return the next token as a string slice. Panic if input is exhausted.
  #[inline]
  pub fn next_str(&mut self) -> &str {
    self.token().expect("Input exhausted")
  }

  /// Return the next line as a string slice. Panic if input is exhausted.
  #[inline]
  pub fn next_line(&mut self) -> &str {
    self.skip_whitespace();

    let start = self.pos;
    while self.pos < self.buf.len() && self.buf[self.pos] != b'\n' {
      self.pos += 1;
    }

    unsafe {
      // SAFETY: The input is valid UTF-8, and we only slice within it.
      from_utf8_unchecked(&self.buf[start..self.pos])
    }
  }

  /// Return the next byte from the input, skipping any leading whitespace.
  #[inline]
  pub fn next_byte(&mut self) -> u8 {
    self.skip_whitespace();

    let byte = self.buf[self.pos];
    self.pos += 1;
    byte
  }

  /// Skip over any whitespace characters in the input.
  fn skip_whitespace(&mut self) {
    while self.pos < self.buf.len() && self.buf[self.pos].is_ascii_whitespace() {
      self.pos += 1;
    }
  }

  /// Return the next token as a string slice, or `None` if input is exhausted.
  fn token(&mut self) -> Option<&str> {
    self.skip_whitespace();

    if self.pos >= self.buf.len() {
      return None;
    }

    let start = self.pos;
    while self.pos < self.buf.len() && !self.buf[self.pos].is_ascii_whitespace() {
      self.pos += 1;
    }

    unsafe {
      // SAFETY: The input is valid UTF-8, and we only slice within it.
      Some(from_utf8_unchecked(&self.buf[start..self.pos]))
    }
  }
}
