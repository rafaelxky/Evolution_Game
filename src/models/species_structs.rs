use serde::{Serialize, Deserialize};
use std::{cell::RefCell, rc::Rc};

use crate::{services::rng_services::*};
use crate::models::colors::Colors;
use crate::models::diets::Diet;

#[derive(Serialize, Deserialize, Clone)]
pub struct Specie {
    pub id: u32,
    pub diet: Diet,
    pub hunger_regen: u16,
    pub hunger_degen: u16,
    pub speed: u8,
    pub start_pop: u8,
    pub color: Colors,
    
}
impl Specie{
    pub fn new(id: u32, diet: Diet, speed: u8, hunger_regen: u16, hunger_degen: u16, start_pop: u8, color: Colors) -> Self{
        Specie {
            id: id,
            diet: diet,
            speed: speed,
            hunger_regen: hunger_regen,
            hunger_degen: hunger_degen,
            start_pop: start_pop,
            color: color,
        }
    }
    pub fn random(id: u32) -> Self {
        Specie {
            id: id,
            diet: Diet::random(), 
            speed: random(10,100),
            hunger_regen: random(10, 100) as u16,
            hunger_degen: random(10, 100) as u16,
            start_pop: random(1, 10),
            color: Colors::get_random(),
        }
    }
    pub fn print(&self){
        println!("{} {} {} {}", self.diet, self.speed, self.hunger_regen, self.color);
    }
    pub fn pop_species(ammount: u32) -> Vec<Rc<RefCell<Specie>>>{
        let mut species: Vec<Rc<RefCell<Specie>>> = Vec::new();
        for i in 1..=ammount {
            species.push(Rc::from(RefCell::new(Specie::random(i))));
        }
    return species;
    }

    pub fn pop_species_from_seed(seed: Vec<Rc<RefCell<Specie>>>) -> Vec<Rc<RefCell<Specie>>>{
        let id = 0;

        let new_gen: Vec<Rc<RefCell<Specie>>> = seed.iter().map(|specie| {
            if random(0, 100) > 95 {
                let previous = specie.borrow().diet.to_string();
                specie.borrow_mut().diet = Diet::random();
                println!("diet mutation in specie {} from {} to {}", specie.borrow().id, previous, specie.borrow().diet);
            }
            if random(0, 100) > 90 {
                let previous = specie.borrow().hunger_degen;
                let mutation = (specie.borrow_mut().hunger_degen as i8 + random_signed(-5, 5)) as u16;
                specie.borrow_mut().hunger_degen = mutation;
                println!("hunger_degen mutation in specie {} from {} to {}", specie.borrow().id, previous, specie.borrow().hunger_degen);
            }
            if random(0, 100) > 90 {
                let previous = specie.borrow().hunger_regen;
                let mutation = (specie.borrow_mut().hunger_regen as i8 + random_signed(-5, 5)) as u16;
                specie.borrow_mut().hunger_regen = mutation;
                println!("hunger_regen mutation in specie {} from {} to {}", specie.borrow().id, previous, specie.borrow().hunger_regen);
            }
            return specie.clone();
        }).collect();

        return new_gen
    }

}
