use std::{cell::RefCell, rc::Rc, sync::Mutex};
use once_cell::sync::Lazy;
use crate::models::biomes::Biomes;
use crate::models::species_structs::*;
use crate::services::rng_services::*;
use crate::models::animal_struct::Animal;

pub static Biome: Lazy<Mutex<Biomes>> = Lazy::new(|| Mutex::new(Biomes::get_random()));

pub fn pop_animals(species: Vec<Rc<RefCell<Specie>>>)-> Vec<Rc<RefCell<Animal>>>{
    let mut index: u32 = 0;
    let mut animals: Vec<Rc<RefCell<Animal>>> = Vec::new();
    for specie in species {
        for _ in 1..=specie.borrow_mut().start_pop {
            animals.push(Rc::new(RefCell::new(Animal::new(specie.clone(), index))));
            index = index + 1;
        }
    }
    return animals;
}

pub fn was_eaten(prey: &Animal, predator: &Animal) -> bool{
    let prey_specie = prey.specie.borrow();
    let predator_specie= predator.specie.borrow();

    let prey_speed = prey_specie.speed;
    let predator_speed = predator_specie.speed;
    let prey_color = &prey_specie.color;
    let predator_color = &predator_specie.color;
    let biome = Biome.lock().unwrap();

    let chance = 50 + (predator_speed as i8 - prey_speed as i8) / 2 + (biome.calc_camouflage(predator_color) - biome.calc_camouflage(prey_color));
    if chance > random(0, 100) as i8 {
        return true;
    }
    false
}
