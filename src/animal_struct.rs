use std::{cell::RefCell, rc::Rc};
use colored::{Colorize};

use crate::{rng::*};
use crate::status::Status;

use crate::species_structs::Specie;

pub struct AnimalMut{
    pub animal: Animal,
    pub hunger_regen: u16,
    pub hunger_degen: u16,
    pub speed: u16,
}
impl AnimalMut {
    pub fn new(specie: Rc<RefCell<Specie>>, index: u32) -> Self{
        AnimalMut { 
            animal: Animal::new(Rc::clone(&specie), index), 
            hunger_regen: (specie.borrow().hunger_regen as i8 + random_signed(-4, 4)) as u16, 
            hunger_degen: (specie.borrow().hunger_degen as i8 + random_signed(-4, 4)) as u16, 
            speed:  (specie.borrow().speed as i8 + random_signed(-4, 4)) as u16, 
        }
    }
}

pub struct Animal {
    pub specie: Rc<RefCell<Specie>>,
    pub status: Status,
    pub hunger: u16,
    pub id: u32,
    pub death_reason: Option<String>,
}
impl Animal {
    pub fn new(specie: Rc<RefCell<Specie>>, index: u32) -> Self{
        Animal {
            specie: specie,  
            status: Status::Alive,
            hunger: 100,
            id: index,
            death_reason: None, 
        }
    }
    pub fn print(&self){
        print!("animal {} is a {} and is {} ",self.id.to_string().truecolor(0, 255, 255), self.specie.borrow().diet, self.status);
        match &self.death_reason {
            Some(death_reason) => {print!("due to {}\n", death_reason);},
            None => {print!("\n");},
        }
        println!("specie{}, speed {}, hunger {}, hunger_Regen {}, hunger_Degen {}, start_pop {}", 
        self.specie.borrow().id.to_string().truecolor(255, 0, 255),
        self.specie.borrow().speed.to_string().truecolor(255, 0, 255), 
        self.hunger.to_string().truecolor(255, 0, 255),
        self.specie.borrow().hunger_regen.to_string().truecolor(255, 0, 255),
        self.specie.borrow().hunger_degen.to_string().truecolor(255, 0, 255),
        self.specie.borrow().start_pop.to_string().truecolor(255, 0, 255),
    );
    }
}
