use std::fs::{OpenOptions};
use std::io::{Write};
use std::path::{Path};
use std::{cell::RefCell, rc::Rc};

use crate::models::species_structs::*;

pub fn write_to_file<P: AsRef<Path>>(path: P, content: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(false)
        .create(true)
        .open(path)
        .expect("Failed to open file");
    file.write_all(content.as_bytes()).unwrap();
}

pub fn read_entire_file<P: AsRef<Path>>(path: P) -> String {
    std::fs::read_to_string(path).unwrap()
}

pub fn specie_from_file<P: AsRef<Path>>(path: P) -> Vec<Rc<RefCell<Specie>>> {
    let data = read_entire_file(path);
    let species_vec: Vec<Specie> = serde_json::from_str(&data).expect("JSON error");
    species_vec.into_iter()
        .map(|specie| Rc::new(RefCell::new(specie)))
        .collect()
}