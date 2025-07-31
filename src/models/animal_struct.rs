use std::{cell::RefCell, rc::Rc};
use colored::{Colorize};
use crate::models::death_reasons::DeathReason;
use crate::models::diets::Diet;
use crate::models::status::Status;

use crate::models::species_structs::Specie;

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
        print!("animal {} is a {} {} and is {} ",self.id.to_string().truecolor(0, 255, 255), self.specie.borrow().color ,self.specie.borrow().diet, self.status);
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

    pub fn starve(&mut self){
        println!("Animal {} starved to death!", self.id.to_string().truecolor(0, 255, 255));
        self.death_reason = Some("starvation".to_string());
        self.status = Status::Dead;
        self.hunger = 0;
    }
    pub fn print_hunger(&self){
        println!("Animal {} hunger is now {}",
        self.id.to_string().truecolor(0, 255, 255), 
        self.hunger.to_string().truecolor(255, 0, 255));
    }

    pub fn isAlive(&self) -> bool{
        self.death_reason.is_none()
    }
    pub fn isCarnivore(&self) -> bool {
        self.specie.borrow().diet == Diet::Carnivore
    } 
    pub fn isHerbivore(&self) -> bool {
        self.specie.borrow().diet == Diet::Vegetarian
    }
    pub fn isOmnivore(&self) -> bool {
        self.specie.borrow().diet == Diet::Omnivore
    }
}
