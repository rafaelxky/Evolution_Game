use core::fmt;
use colored::{Colorize};


#[derive(PartialEq)]
pub enum Status {
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
