use rand::{rngs::OsRng, seq::SliceRandom, Rng};
use zeroize::Zeroizing;

use crate::error::{Result, VaultError};

const LOWER: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &str = "0123456789";
const SYMBOLS: &str = "!@#$%^&*()-_=+[]{};:,.<>/?";
const AMBIGUOUS: &str = "O0Il1|";

pub const MAX_LENGTH: usize = 256;

#[derive(Clone, Debug)]
pub struct GeneratorOptions {
    pub length: usize,
    pub lowercase: bool,
    pub uppercase: bool,
    pub digits: bool,
    pub symbols: bool,
    pub exclude_ambiguous: bool,
}

impl Default for GeneratorOptions {
    fn default() -> Self {
        Self { length: 20, lowercase: true, uppercase: true, digits: true, symbols: true, exclude_ambiguous: false }
    }
}

/// Generates a password from the OS CSPRNG. Every selected character set is
/// guaranteed to appear at least once.
pub fn generate_password(opts: &GeneratorOptions) -> Result<String> {
    let sets: Vec<Vec<char>> = [
        (opts.lowercase, LOWER),
        (opts.uppercase, UPPER),
        (opts.digits, DIGITS),
        (opts.symbols, SYMBOLS),
    ]
    .into_iter()
    .filter(|(on, _)| *on)
    .map(|(_, s)| s.chars().filter(|c| !(opts.exclude_ambiguous && AMBIGUOUS.contains(*c))).collect())
    .collect();

    if sets.is_empty() {
        return Err(VaultError::InvalidGeneratorOptions("select at least one character set"));
    }
    if opts.length < sets.len() {
        return Err(VaultError::InvalidGeneratorOptions("length is shorter than the number of selected sets"));
    }
    if opts.length > MAX_LENGTH {
        return Err(VaultError::InvalidGeneratorOptions("length exceeds the maximum of 256"));
    }

    let pool: Vec<char> = sets.iter().flatten().copied().collect();
    let mut chars: Zeroizing<Vec<char>> = Zeroizing::new(Vec::with_capacity(opts.length));
    for set in &sets {
        chars.push(set[OsRng.gen_range(0..set.len())]);
    }
    while chars.len() < opts.length {
        chars.push(pool[OsRng.gen_range(0..pool.len())]);
    }
    chars.shuffle(&mut OsRng);
    Ok(chars.iter().collect())
}
