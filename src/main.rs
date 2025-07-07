use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::{self, Path};
use serde::{Serialize, Deserialize};
use core::fmt;
use std::{cell::RefCell, fmt::format, iter::Cycle, rc::Rc, sync::Mutex};
use colored::{Colorize};

use rand::{rand_core::le, Rng};

static CYCLES: Mutex<u32> = Mutex::new(0);

fn main() {
//let species: Vec<Rc<RefCell<Specie>>> = pop_species(random(1, 10) as u32);
//let species = specie_from_file("species.json");
let species = pop_species_from_seed(specie_from_file("species.json"));

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
                // Skip if not carnivore or not alive
                if animal.specie.borrow().diet != Diet::Carnivore || animal.status != Status::Alive {
                    continue;
                }
            } // drop immutable borrow here

            // Mutable borrow the carnivore
            let mut animal = animal_rc.borrow_mut();

            // Try to eat a herbivore first
            let mut ate = false;
            for j in 0..len {
                if i == j { continue; }

                let mut other = animals[j].borrow_mut();

                if other.specie.borrow().diet == Diet::Vegetarian && other.status == Status::Alive {
                    // if carnivore is able to catch vegetarian
                    if was_eaten(other.specie.borrow().speed, animal.specie.borrow().speed){
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
                    // if vegetarian is able to run
                    println!("Animal {} ran from animal {}",
                    other.id.to_string().truecolor(0, 255, 255),
                    animal.id.to_string().truecolor(0, 255, 255),
                    );

                    let total_hunger: i16 = (animal.hunger as i16 - animal.specie.borrow().hunger_degen as i16);
                    // if hunger drops bellow 0 die
                    if total_hunger < 0 {
                        println!("Animal {} starved to death!", animal.id.to_string().truecolor(0, 255, 255));
                        animal.death_reason = Some("starvation".to_string());
                        animal.status = Status::Dead;
                        animal.hunger = 0;
                        ate = true;
                        break;
                    }
                    // subtract hunger
                    animal.hunger = total_hunger as u16;
                    println!("Animal {} hunger is now {}",
                    animal.id.to_string().truecolor(0, 255, 255), 
                    animal.hunger.to_string().truecolor(255, 0, 255));
                    ate = true;
                    break;
                }
            }

            // If no herbivore to eat, try to eat another carnivore (go mad)
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

        // If no more herbivores alive -> stop
        let herbivore_alive = animals.iter().any(|a| {
            let a = a.borrow();
            a.specie.borrow().diet == Diet::Vegetarian && a.status == Status::Alive
        });

        // If no carnivores alive -> stop
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
            // If carnivores alive, loop continues to allow carnivores eating carnivores
        }

        if !carnivore_alive {
            println!("No carnivores alive. Ending.");
            break;
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct Specie {
    id: u32,
    diet: Diet,
    hunger_regen: u16,
    hunger_degen: u16,
    speed: u8,
    start_pop: u8,
}
impl Specie{
    fn new(id: u32, diet: Diet, speed: u8, hunger_regen: u16, hunger_degen: u16, start_pop: u8) -> Self{
        Specie {
            id: id,
            diet: diet,
            speed: speed,
            hunger_regen: hunger_regen,
            hunger_degen: hunger_degen,
            start_pop: start_pop,
        }
    }
    fn random(id: u32) -> Self {
        Specie {
            id: id,
            diet: Diet::random(), 
            speed: random(10,100),
            hunger_regen: random(10, 100) as u16,
            hunger_degen: random(10, 100) as u16,
            start_pop: random(1, 10),
        }
    }
    fn print(&self){
        println!("{} {} {}", self.diet, self.speed, self.hunger_regen);
    }
}

fn random(min: u8, max: u8) -> u8 {
    let mut rng = rand::rng();
    return rng.random_range(min..max);
}

struct Animal {
    specie: Rc<RefCell<Specie>>,
    status: Status,
    hunger: u16,
    id: u32,
    death_reason: Option<String>,
}
impl Animal {
    fn new(specie: Rc<RefCell<Specie>>, index: u32) -> Self{
        Animal {
            specie: specie,  
            status: Status::Alive,
            hunger: 100,
            id: index,
            death_reason: None, 
        }
    }
    fn print(&self){
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

#[derive(PartialEq, Serialize, Deserialize, Clone)]
enum Diet{
    Carnivore,
    Vegetarian, 
    Omnivore,
}
impl fmt::Display for Diet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Diet::Carnivore => "carnivore".truecolor(255, 0, 0),
            Diet::Vegetarian => "vegetarian".truecolor(0, 255, 0),
            Diet::Omnivore => "omnivore".truecolor(255, 255, 0),
        };
        write!(f, "{}", s)
    }
}

impl  Diet {
    fn random() -> Self{
        let mut rng = rand::rng();
        match rng.random_range(1..=3) {
            1 => Diet::Carnivore,
            2 => Diet::Omnivore,
            3 => Diet::Vegetarian,
            _ => Diet::Carnivore,
        }
    }
}

#[derive(PartialEq)]
enum Status {
    Alive,
    Dead,
}
impl fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Status::Alive => "alive".truecolor(0, 255, 0),
            Status::Dead => "dead".truecolor(255, 0, 0),
        };
        write!(f, "{}",s)
    }
    
}

fn pop_species(ammount: u32) -> Vec<Rc<RefCell<Specie>>>{
    let mut species: Vec<Rc<RefCell<Specie>>> = Vec::new();
    for i in 1..=ammount {
        species.push(Rc::from(RefCell::new(Specie::random(i))));
    }
    return species;
}
fn pop_species_from_seed(seed: Vec<Rc<RefCell<Specie>>>) -> Vec<Rc<RefCell<Specie>>>{
    let id = 0;

    let new_gen: Vec<Rc<RefCell<Specie>>> = seed.iter().map(|specie| {
        // diet mutation
        if random(0, 100) > 95 {
            let previous = specie.borrow().diet.to_string();
            specie.borrow_mut().diet = Diet::random();
            println!("diet mutation in specie {} from {} to {}", specie.borrow().id, previous, specie.borrow().diet);
        }
        // hunger degen mutation
        if random(0, 100) > 90 {
            let previous = specie.borrow().hunger_degen;
            let mutation = (specie.borrow_mut().hunger_degen as i8 + random_signed(-5, 5)) as u16;
            specie.borrow_mut().hunger_degen = mutation;
            println!("hunger_degen mutation in specie {} from {} to {}", specie.borrow().id, previous, specie.borrow().hunger_degen);
        }
        // hunger regen mutation
        if random(0, 100) > 90 {
            let previous = specie.borrow().hunger_regen;
            let mutation = (specie.borrow_mut().hunger_regen as i8 + random_signed(-5, 5)) as u16;
            specie.borrow_mut().hunger_regen = mutation;
            println!("hunger_regen mutation in specie {} from {} tp {}", specie.borrow().id, previous, specie.borrow().hunger_regen);
        }
        return specie.clone();
    }).collect();

    return new_gen
}

fn random_signed(min: i8, max: i8) -> i8{
    let mut rng = rand::rng();
    return rng.random_range(min..=max);
}

fn pop_animals(species: Vec<Rc<RefCell<Specie>>>)-> Vec<Rc<RefCell<Animal>>>{
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

fn was_eaten(prey_speed: u8, predator_speed: u8) -> bool{
    let chance = 50 + (predator_speed as i8 - prey_speed as i8) / 2;
    if chance > random(0, 100) as i8 {
        return true;
    }
    false
}


fn append_to_file<P: AsRef<Path>>(path: P, content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)?;
    file.write_all(content.as_bytes())?;
    println!("Successfully appended to file.");
    Ok(())
}

fn write_to_file<P: AsRef<Path>>(path: P, content: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(false)
        .create(true)
        .open(path)
        .expect("Failed to open file");
    file.write_all(content.as_bytes()).unwrap();
}

fn read_entire_file<P: AsRef<Path>>(path: P) -> String {
    std::fs::read_to_string(path).unwrap()
}

fn specie_from_file<P: AsRef<Path>>(path: P) -> Vec<Rc<RefCell<Specie>>> {
    let data = read_entire_file(path);
    let species_vec: Vec<Specie> = serde_json::from_str(&data).expect("JSON error");
    species_vec.into_iter()
        .map(|specie| Rc::new(RefCell::new(specie)))
        .collect()
}