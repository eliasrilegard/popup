use crate::algebra::{Complex, fft, ifft};

/// Compute the convolution of two integer coefficient vectors using FFT.
///
/// Return the coefficients of the product polynomial.
///
/// # Complexity
/// * Time complexity: *O((n + m) log(n + m))*.
/// * Auxiliary space: *O(n + m)*.
///
/// where *n* and *m* are the lengths of the input vectors.
pub fn convolve(p1: &[i64], p2: &[i64]) -> Vec<i64> {
  let n = p1.len();
  let m = p2.len();

  // Determine smallest power of 2 that can hold the convolution
  let mut size = 1;
  while size < n + m - 1 {
    size <<= 1;
  }

  // Convert input to complex numbers and pad with zeros
  let mut a: Vec<_> = (0..n).map(|i| Complex::new(p1[i] as f64, 0.0)).collect();
  let mut b: Vec<_> = (0..m).map(|i| Complex::new(p2[i] as f64, 0.0)).collect();
  a.resize(size, Complex::ZERO);
  b.resize(size, Complex::ZERO);

  let a_fft = fft(&a);
  let b_fft = fft(&b);

  // Convolution in the time domain corresponds to (pointwise) multiplication
  // in the frequency domain
  let c_fft: Vec<_> = a_fft
    .iter()
    .zip(b_fft.iter())
    .map(|(&x, &y)| x * y)
    .collect();

  let c = ifft(&c_fft);

  // The coefficients of the resulting polynomial are the real parts of the
  // inverse FFT (within rounding error)
  let mut result: Vec<_> = c.iter().map(|z| z.re.round() as i64).collect();

  // Remove trailing zeros
  while result.len() > 1 && *result.last().unwrap() == 0 {
    result.pop();
  }

  result
}
