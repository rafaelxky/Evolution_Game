use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::{self, Path};
use serde::{Serialize, Deserialize};
use core::fmt;
use std::{cell::RefCell, fmt::format, iter::Cycle, rc::Rc, sync::Mutex};
use colored::{Colorize};
use rand::{rand_core::le, Rng};

use crate::species_structs::*;
use crate::rng::*;


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

pub fn was_eaten(prey_speed: u8, predator_speed: u8) -> bool{
    let chance = 50 + (predator_speed as i8 - prey_speed as i8) / 2;
    if chance > random(0, 100) as i8 {
        return true;
    }
    false
}
