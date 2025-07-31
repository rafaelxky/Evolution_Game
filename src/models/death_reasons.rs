pub enum DeathReason{
    Canibalized,
    NoFood,
    Eaten,
    None,
}

// todo: implement the rest
impl DeathReason {
    pub fn value(&self) -> &'static str {
        match self {
            DeathReason::Canibalized => "",
            DeathReason::NoFood => "",
            DeathReason::Eaten => "",
            DeathReason::None => "",
        }
    }   
}