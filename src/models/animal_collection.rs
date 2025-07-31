use std::{cell::RefCell, rc::Rc};

use crate::models::animal_struct::Animal;

pub struct AnimalCollection {
    animals: Vec<Rc<RefCell<Animal>>>,
}

impl AnimalCollection {
    // todo: implement a wraper collection for animals Vec  
}