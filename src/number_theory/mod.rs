//! Number theory algorithms.

mod congruence;
mod euclidean;
mod modular;
mod prime;
mod recurrence;

pub use congruence::{Congruence, solve_crt};
pub use euclidean::{egcd, gcd, lcm};
pub use modular::{mod_div, mod_inv, mod_mul};
pub use prime::Eratosthenes;
pub use recurrence::AffineRecurrence;
