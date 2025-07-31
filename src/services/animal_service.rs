use crate::models::animal_struct::Animal;
use colored::Colorize;

pub fn print_mad(animal: &Animal, other: &Animal){
    println!(
        "Carnivore {} went mad and ate carnivore {}",
        animal.id.to_string().truecolor(0, 255, 255),
        other.id.to_string().truecolor(0, 255, 255)
    );
}

    
pub fn print_ran_from(animal: &Animal, other: &Animal){
    println!("Animal {} ran from animal {}",
        other.id.to_string().truecolor(0, 255, 255),
        animal.id.to_string().truecolor(0, 255, 255),
    );
}

pub fn print_starved(animal: &Animal){
    println!("Carnivore {} found no food to eat and starved!", animal.id.to_string().truecolor(0, 255, 255));
}

pub fn calc_hunger(animal: &Animal) -> i16{
    animal.hunger as i16 - animal.specie.borrow().hunger_degen as i16
}