use serde::{Serialize, Deserialize};
use core::fmt;
use colored::{Colorize};
use rand::{Rng};

#[derive(PartialEq, Serialize, Deserialize, Clone)]
pub enum Diet{
    Carnivore,
    Vegetarian, 
    Omnivore,
}
impl fmt::Display for Diet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Diet::Carnivore => "carnivore".truecolor(255, 0, 0),
            Diet::Vegetarian => "vegetarian".truecolor(0, 255, 0),
            Diet::Omnivore => "omnivore".truecolor(255, 255, 0),
        };
        write!(f, "{}", s)
    }
}

impl  Diet {
    pub fn random() -> Self{
        let mut rng = rand::rng();
        match rng.random_range(1..=3) {
            1 => Diet::Carnivore,
            2 => Diet::Omnivore,
            3 => Diet::Vegetarian,
            _ => Diet::Carnivore,
        }
    }
}
