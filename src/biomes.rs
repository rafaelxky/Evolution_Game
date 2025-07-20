use core::fmt;
use colored::Color;
use rand::Rng;
use crate::colors::Colors;

#[derive(PartialEq, Debug)]
pub enum Biomes{
    DESERT,
    SNOW,
    TAIGA,
}

impl Biomes {
    pub fn get_random() -> Self {
        let mut rng = rand::rng();
        match rng.random_range(0..3) {
            0 => Biomes::DESERT,
            1 => Biomes::SNOW,
            _ => Biomes::TAIGA,
        }
    }
    pub fn get_matching_color(&self) -> Colors{
        match self {
            Biomes::DESERT => Colors::YELLOW,
            Biomes::SNOW => Colors::WHITE,
            Biomes::TAIGA => Colors::GREEN,
        }
    }
    
    pub fn calc_camouflage(&self, camo: &Colors) -> i8 {
        let rate = 20;
        if self.get_matching_color() == *camo {
            return rate;
        }
        return 0;
    }
}



impl fmt::Display for Biomes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let biome_str = match &self {
            Biomes::DESERT => "desert",
            Biomes::SNOW => "snow",
            Biomes::TAIGA => "taiga",
        };
        write!(f, "{}", biome_str)
    }
}
