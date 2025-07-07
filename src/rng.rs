use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::{self, Path};
use serde::{Serialize, Deserialize};
use core::fmt;
use std::{cell::RefCell, fmt::format, iter::Cycle, rc::Rc, sync::Mutex};
use colored::{Colorize};
use rand::{rand_core::le, Rng};

pub fn random_signed(min: i8, max: i8) -> i8{
    let mut rng = rand::rng();
    return rng.random_range(min..=max);
}

pub fn random(min: u8, max: u8) -> u8 {
    let mut rng = rand::rng();
    return rng.random_range(min..max);
}