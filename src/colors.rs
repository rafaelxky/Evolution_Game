use core::fmt;

use rand::Rng;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Colors {
    GREEN,
    WHITE,
    YELLOW,
    RAINBOW,
}

impl Colors {
    pub fn get_random() -> Self {
        let mut rng = rand::rng();
        match rng.random_range(0..4) {
            0 => Colors::GREEN,
            1 => Colors::WHITE,
            2 => Colors::YELLOW,
            _ => Colors::RAINBOW,
        }
    }
}

impl fmt::Display for Colors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color_str = match self {
            Colors::GREEN => "green",
            Colors::WHITE => "white",
            Colors::YELLOW => "yellow",
            Colors::RAINBOW => "rainbow",
        };
        write!(f, "{}", color_str)
    }
}

