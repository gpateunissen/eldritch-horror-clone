use super::investigator_instance::InvestigatorInstance;

#[derive(Debug)]
pub struct GameState {
    pub investigators: Vec<InvestigatorInstance>,
    pub current_turn: usize,
}

impl GameState {

    pub fn new() -> Self {
        Self {
            investigators: Vec::new(),
            current_turn: 0,
        }
    }

    pub fn add_investigator(&mut self, investigator: InvestigatorInstance) {
        self.investigators.push(investigator);
    }

}