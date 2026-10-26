const WORD_BITS: usize = u64::BITS as usize;

/// Store a sequence of bits in a compact representation.
///
/// Each bit occupies one bit of storage, rather than a full byte or larger
/// integer.
#[derive(Debug, Default)]
pub struct BitVec {
  data: Vec<u64>,
  len: usize,
}

impl BitVec {
  /// Create an empty `BitVec`.
  pub fn new() -> Self {
    Self {
      data: Vec::new(),
      len: 0,
    }
  }

  /// Create an empty `BitVec` with capacity for at least `capacity` bits.
  pub fn with_capacity(capacity: usize) -> Self {
    let words = capacity.div_ceil(WORD_BITS);
    let data = Vec::with_capacity(words);
    Self { data, len: 0 }
  }

  /// Create a `BitVec` of length `bits`, initialized to zero.
  pub fn zeros(bits: usize) -> Self {
    let words = bits.div_ceil(WORD_BITS);
    let data = vec![0; words];
    Self { data, len: bits }
  }

  /// Create a `BitVec` of length `bits`, initialized to one.
  pub fn ones(bits: usize) -> Self {
    let words = bits.div_ceil(WORD_BITS);
    let mut data = vec![!0; words];

    let excess = words * WORD_BITS - bits;
    if excess > 0 {
      let mask = !0 >> excess;
      data[words - 1] &= mask;
    }

    Self { data, len: bits }
  }

  /// Return the number of bits in the vector.
  pub fn len(&self) -> usize {
    self.len
  }

  /// Return whether the vector contains no bits.
  pub fn is_empty(&self) -> bool {
    self.len == 0
  }

  /// Set the bit at `index` to one.
  pub fn set(&mut self, index: usize) {
    assert!(index < self.len);

    let (word, bit) = locate(index);
    self.data[word] |= 1 << bit;
  }

  /// Append `bit` to the end of the vector.
  pub fn push(&mut self, bit: bool) {
    if self.len.is_multiple_of(WORD_BITS) {
      self.data.push(0);
    }
    self.len += 1;

    if bit {
      self.set(self.len - 1);
    }
  }

  /// Return the value of the bit at `index`.
  pub fn get(&self, index: usize) -> bool {
    assert!(index < self.len);

    let (word, bit) = locate(index);
    self.data[word] >> bit & 1 == 1
  }

  /// Set the bit at `index` to zero.
  pub fn unset(&mut self, index: usize) {
    assert!(index < self.len);

    let (word, bit) = locate(index);
    self.data[word] &= !(1 << bit);
  }

  /// Count the number of set bits in the vector.
  #[doc(alias = "popcount")]
  pub fn count_ones(&self) -> usize {
    self.data.iter().map(|x| x.count_ones() as usize).sum()
  }

  /// Count the number of unset bits in the vector.
  pub fn count_zeros(&self) -> usize {
    self.len - self.count_ones()
  }

  /// Remove all bits from the vector.
  pub fn clear(&mut self) {
    self.data.clear();
    self.len = 0;
  }

  /// Return an iterator over the bits in the vector.
  pub fn iter(&self) -> impl Iterator<Item = bool> {
    (0..self.len).map(move |i| self.get(i))
  }
}

/// Locate the word and bit containing `index`.
fn locate(index: usize) -> (usize, usize) {
  let word = index / WORD_BITS;
  let bit = index % WORD_BITS;
  (word, bit)
}
