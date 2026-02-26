#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub health: i32,
    pub sanity: i32,
}

impl Player {
    pub fn new(name: String) -> Self {
        Self { 
            name, 
            health: 5,
            sanity: 5,
        }
    }
}