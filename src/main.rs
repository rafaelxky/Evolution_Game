use std::{cell::RefCell, rc::Rc, sync::Mutex};
use colored::{Colorize};

mod services;
use services::file_service::*;
use crate::models::species_structs::*;
use crate::services::{animal_service, ecosystem_service::*};

use crate::models::diets::Diet;
use crate::models::status::Status;
mod models;
use crate::models::animal_struct::Animal;
use crate::services::animal_service::*;

static CYCLES: Mutex<u32> = Mutex::new(0);

fn main() {
    
println!("Biome {}", services::ecosystem_service::Biome.lock().unwrap());

let species = Specie::pop_species_from_seed(specie_from_file("species.json"));

//let species: Vec<Rc<RefCell<Specie>>> = Specie::pop_species(random(1, 10) as u32);

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
                if !animal.is_carnivore() || !animal.is_alive() {
                    continue;
                }
            } 

            let mut animal = animal_rc.borrow_mut();

            let mut ate = false;
            for j in 0..len {
                if i == j { continue; }

                let mut other = animals[j].borrow_mut();

                if animal.can_eat(&other) {
                    
                    if animal.try_eat(&mut other) {
                        ate = true;
                        break;
                    }

                    animal_service::print_ran_from(&animal, &other);

                    let total_hunger = animal_service::calc_hunger(&animal);
                    if total_hunger < 0 {
                       animal.starve();
                        ate = true;
                    }
                    animal.hunger = total_hunger as u16;
                    animal.print_hunger();
                    ate = true;
                    break;
                }
            }

            if !ate {
                for j in 0..len {
                    if i == j { continue; }

                    let mut other = animals[j].borrow_mut();

                    if other.is_carnivore() && other.is_alive() {

                        animal_service::print_mad(&animal, &other);

                        other.death_reason = Some(format!("behing canibalized by animal {}", animal.id.to_string().truecolor(0, 255, 255)));
                        other.status = Status::Dead;
                        animal.hunger += 50;
                        ate = true;
                        break;
                    }
                }
            }

            if !ate {
                animal_service::print_starved(&animal);
                animal.status = Status::Dead;
                animal.death_reason = Some("no more available food".to_string());
                return;
            }

        }

        let herbivore_alive = animals.iter().any(|a| {
            let a = a.borrow();
            a.is_herbivore() && a.is_alive()
        });

        let carnivore_alive = animals.iter().any(|a| {
            let a = a.borrow();
            a.is_carnivore() && a.is_alive()
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



