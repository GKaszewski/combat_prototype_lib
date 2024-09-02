use crate::battle::{Battle, Player};

pub struct Game {
    pub battle: Battle,
    pub players: Vec<Player>,
    pub current_player: usize,
}

impl Game {
    pub fn new(battle: Battle, players: Vec<Player>) -> Self {
        Self {
            battle,
            players,
            current_player: 0,
        }
    }

    pub fn next_turn(&mut self) {
        self.current_player = (self.current_player + 1) % self.players.len();
    }

    pub fn game_loop(&mut self) {
        self.battle.start();
    }
}
