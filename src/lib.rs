//! # Coin Flip
//!
//! A library for flipping a coin and getting the result. Depends on rand "0.9.2"
use rand::prelude::*;

/// Enum that defines the two coinflip variants 
/// Heads and Tails with discriminants
#[derive(PartialEq)]
pub enum Coin {
    Heads = 0,
    Tails,
}

impl TryFrom<u8> for Coin {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Coin::Heads),
            1 => Ok(Coin::Tails),
            _ => Err(()),
        }
    }
}

/// Flips a coin and returns the result
/// takes an FnMut handler function that takes the result
/// returns the result from the function
/// 
/// # Examples
/// ```
/// let mut heads_results: u32 = 0;
/// let mut tails_results: u32 = 0;
///
/// let mut handler = |r: &Coin| {
///     if *r == Coin::Heads {
///         heads_results += 1;
///     } else {
///         tails_results += 1;
///     }
/// };
/// for _ in 1..=total_runs {
///     flip_coin(&mut handler);
/// }
///```
pub fn flip_coin<F: FnMut(&Coin)>(handler: &mut F) -> Coin {
    let mut rng = rand::rng();
    let random = rng.random_range::<u8, _>(0..2);
    let res = <Coin as TryFrom<u8>>::try_from(random).unwrap();
    handler(&res);
    res
}

pub fn calculate_percentage(total: u64, part: u32) -> f64 {
    if total == 0 {
        return 0.0;
    }
    (f64::from(part) / total as f64) * 100.0
}
