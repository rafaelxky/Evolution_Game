use core::fmt;
use rand::Rng;

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
