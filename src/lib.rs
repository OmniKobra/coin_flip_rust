use rand::prelude::*;

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

pub fn flip_coin<F: FnMut(&Coin)>(rng: &mut ThreadRng, handler: &mut F) -> Coin {
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
