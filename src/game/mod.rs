use crate::battle::{Action, Battle, BattleState, Player};

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
        self.battle.current_turn = (self.battle.current_turn + 1) % self.players.len();
        let next_turn = &mut self.battle.turns[self.battle.current_turn as usize];

        next_turn.finished = false;
        self.battle.state = BattleState::StartedTurn(self.battle.current_turn as u32);
    }

    fn handle_turn(&mut self) {
        let current_turn = &mut self.battle.turns[self.battle.current_turn as usize];
        let current_player = &self.players[self.battle.current_turn as usize];

        println!("Player {}'s turn", current_player.id);

        for action in &current_turn.actions {
            match action {
                Action::Move {
                    unit_id,
                    destination,
                } => {
                    println!(
                        "Player {} moves unit {} to {:?}",
                        current_player.id, unit_id, destination
                    );
                }
                _ => {}
            }
        }

        current_turn.finished = true;
        self.battle.state = BattleState::TurnFinished;
    }

    fn end_turn(&mut self) {
        let current_turn = &mut self.battle.turns[self.battle.current_turn as usize];
        current_turn.finished = true;
        self.battle.state = BattleState::TurnFinished;
    }

    pub fn run(&mut self) {
        if self.battle.state == BattleState::Started {
            self.battle.start();
        }

        loop {
            match self.battle.state {
                BattleState::Finished => {
                    println!(
                        "The battle has finished! Winner: {}",
                        self.battle.winner.as_ref().unwrap()
                    );
                    break;
                }
                BattleState::TurnFinished => {
                    self.end_turn();
                    self.next_turn();
                }
                BattleState::TurnInProgress => {
                    self.handle_turn();
                }
                _ => {}
            }
        }
    }
}
