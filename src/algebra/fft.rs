use std::f64::consts::TAU as TWO_PI;

use crate::algebra::Complex;

/// Recursive radix-2 Cooley–Tukey FFT. Assumes input length is a power of two.
///
/// Based on Lecture 4 of DD2440 Advanced Algorithms, KTH 2025.
///
/// # Complexity
/// * Time complexity: *O(n log n)*.
/// * Auxiliary space: *O(n)*.
pub fn fft(p: &[Complex]) -> Vec<Complex> {
  let n = p.len();
  if n == 1 {
    return vec![p[0]];
  }

  let mut even = Vec::with_capacity(n / 2);
  let mut odd = Vec::with_capacity(n / 2);

  for i in 0..n / 2 {
    even.push(p[2 * i]);
    odd.push(p[2 * i + 1]);
  }

  let even_fft = fft(&even);
  let odd_fft = fft(&odd);

  let mut result = vec![Complex::ZERO; n];

  for j in 0..n / 2 {
    let w = Complex::from_polar(1.0, TWO_PI * j as f64 / n as f64);
    result[j] = even_fft[j] + w * odd_fft[j];
    result[j + n / 2] = even_fft[j] - w * odd_fft[j];
  }

  result
}

/// Inverse FFT.
///
/// Based on Lecture 4 of DD2440 Advanced Algorithms, KTH 2025.
///
/// # Complexity
/// * Time complexity: *O(n log n)*.
/// * Auxiliary space: *O(n)*.
pub fn ifft(p: &[Complex]) -> Vec<Complex> {
  let p_conj: Vec<_> = p.iter().map(|c| c.conj()).collect();

  let mut y = fft(&p_conj);

  let n = p.len() as f64;
  for c in &mut y {
    *c = c.conj() / n;
  }

  y
}
