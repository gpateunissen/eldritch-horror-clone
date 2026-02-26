use super::player::Player;

pub struct GameState {
    pub players: Vec<Player>,
}

impl GameState {
    pub fn new () -> Self {
        Self {
            players: Vec::new(),
        }
    }

    pub fn add_player(&mut self, player: Player){
        self.players.push(player);
    }
}
