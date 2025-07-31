use rand::{Rng};

pub fn random_signed(min: i8, max: i8) -> i8{
    let mut rng = rand::rng();
    return rng.random_range(min..=max);
}

pub fn random(min: u8, max: u8) -> u8 {
    let mut rng = rand::rng();
    return rng.random_range(min..max);
}