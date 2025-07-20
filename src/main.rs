use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::{self, Path};
use serde::{Serialize, Deserialize};
use core::fmt;
use std::{cell::RefCell, fmt::format, iter::Cycle, rc::Rc, sync::Mutex};
use colored::{Colorize};

use rand::{rand_core::le, Rng};

mod file_handler;
use file_handler::*;
mod species_structs;
use species_structs::*;
mod rng;
use rng::*;
mod game;
use game::*;

use crate::diets::Diet;
use crate::biomes::Biomes;
use crate::status::Status;
mod colors;
mod biomes;
mod status;
mod animal_struct;
mod diets;
use crate::animal_struct::Animal;

static CYCLES: Mutex<u32> = Mutex::new(0);

fn main() {
//let species = pop_species_from_seed(specie_from_file("species.json"));

let species: Vec<Rc<RefCell<Specie>>> = Specie::pop_species(random(1, 10) as u32);

let cloned_species: Vec<Specie> = species.iter()
    .map(|specie| specie.borrow().clone())
    .collect();

write_to_file("species.json", &serde_json::to_string_pretty(&cloned_species).unwrap());
println!("{}", serde_json::to_string_pretty(&cloned_species).unwrap());

    let mut animals: Vec<Rc<RefCell<Animal>>> = pop_animals(species);

    print_animals(animals.clone());

    play(animals.clone());
    print_animals(animals.clone());
    println!("Cycles: {}", CYCLES.lock().unwrap());
}

fn print_animals(animals: Vec<Rc<RefCell<Animal>>>){
    animals.iter().for_each(|animal| {animal.borrow().print();});
}

fn play(animals: Vec<Rc<RefCell<Animal>>>) {
    println!("Starting");

    let mut cycles = CYCLES.lock().unwrap();
    loop {
        *cycles = *cycles + 1;

        let len = animals.len();

        for i in 0..len {
            let animal_rc = animals[i].clone();

            {
                let animal = animal_rc.borrow();
                if animal.specie.borrow().diet != Diet::Carnivore || animal.status != Status::Alive {
                    continue;
                }
            } 

            let mut animal = animal_rc.borrow_mut();

            let mut ate = false;
            for j in 0..len {
                if i == j { continue; }

                let mut other = animals[j].borrow_mut();

                if other.specie.borrow().diet == Diet::Vegetarian && other.status == Status::Alive {
                    if was_eaten(&other, &animal){
                    println!(
                        "Animal {} ate animal {}",
                        animal.id.to_string().truecolor(0, 255, 255),
                        other.id.to_string().truecolor(0, 255, 255)
                    );
                    other.death_reason = Some(format!("behing eaten by animal {}", animal.id.to_string().truecolor(0, 255, 255)));
                    other.status = Status::Dead;
                    let hunger_regen = animal.specie.borrow().hunger_regen;
                    animal.hunger += hunger_regen;
                    ate = true;

                    println!("Animal {} hunger is now {}",
                    animal.id.to_string().truecolor(0, 255, 255),
                    animal.hunger.to_string().truecolor(255, 0, 255));
                    
                    break;
                    }
                    println!("Animal {} ran from animal {}",
                    other.id.to_string().truecolor(0, 255, 255),
                    animal.id.to_string().truecolor(0, 255, 255),
                    );

                    let total_hunger: i16 = (animal.hunger as i16 - animal.specie.borrow().hunger_degen as i16);
                    if total_hunger < 0 {
                        println!("Animal {} starved to death!", animal.id.to_string().truecolor(0, 255, 255));
                        animal.death_reason = Some("starvation".to_string());
                        animal.status = Status::Dead;
                        animal.hunger = 0;
                        ate = true;
                        break;
                    }
                    animal.hunger = total_hunger as u16;
                    println!("Animal {} hunger is now {}",
                    animal.id.to_string().truecolor(0, 255, 255), 
                    animal.hunger.to_string().truecolor(255, 0, 255));
                    ate = true;
                    break;
                }
            }

            if !ate {
                for j in 0..len {
                    if i == j { continue; }

                    let mut other = animals[j].borrow_mut();

                    if other.specie.borrow().diet == Diet::Carnivore && other.status == Status::Alive {
                        println!(
                            "Carnivore {} went mad and ate carnivore {}",
                            animal.id.to_string().truecolor(0, 255, 255),
                            other.id.to_string().truecolor(0, 255, 255)
                        );
                        other.death_reason = Some(format!("behing canibalized by animal {}", animal.id.to_string().truecolor(0, 255, 255)));
                        other.status = Status::Dead;
                        animal.hunger += 50;
                        ate = true;
                        break;
                    }
                }
            }

            if !ate {
                println!("Carnivore {} found no food to eat and starved!", animal.id.to_string().truecolor(0, 255, 255));
                animal.status = Status::Dead;
                animal.death_reason = Some("no more available food".to_string());
                return;
            }

        }

        let herbivore_alive = animals.iter().any(|a| {
            let a = a.borrow();
            a.specie.borrow().diet == Diet::Vegetarian && a.status == Status::Alive
        });

        let carnivore_alive = animals.iter().any(|a| {
            let a = a.borrow();
            a.specie.borrow().diet == Diet::Carnivore && a.status == Status::Alive
        });

        if !herbivore_alive {
            println!("No more herbivores alive.");
            if !carnivore_alive {
                println!("No carnivores alive either. Ending.");
                break;
            }
        }

        if !carnivore_alive {
            println!("No carnivores alive. Ending.");
            break;
        }
    }
}



