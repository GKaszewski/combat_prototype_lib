use utils::get_usable_units;

use crate::{
    common::{Grid, Position},
    units::Unit,
};

mod tests;
mod utils;

/// Turn struct - represents a turn in a battle
/// - A turn has a player id - the player that is taking the turn
/// - Actions budget - the number of actions a player can take in a turn (e.g move, attack, etc.)
/// - Actions - list of actions that user decided to take
/// - Finished - true if the turn is finished, false otherwise
/// - Units - list of units that are taking part in the turn
/// - Usable units - list of units that can be used in the turn (e.g units that have the highest initiative value)
pub struct Turn {
    pub player_id: String,
    pub actions_budget: u32,
    pub actions: Vec<Action>,
    pub finished: bool,
    pub units: Vec<Unit>,
    pub usable_units: Vec<Unit>,
}

/// Battle struct - represents a battle between players
/// - A battle has a grid
/// - A battle has players (for now only 2 players are supported)
pub struct Battle {
    pub grid: Box<dyn Grid<Option<Unit>>>,
    pub players: Vec<Player>,
    pub turns: Vec<Turn>,
    pub current_turn: usize,
    pub winner: Option<String>,
}

/// Action enum - represents an action that a unit can perform
pub enum Action {
    /// Move action - moves a unit to a destination
    Move {
        unit_id: String,
        destination: Position,
    },
    /// Attack action - attacks a unit
    Attack {
        attacker_id: String,
        defender_id: String,
    },
    /// Defend action - unit goes into defense mode for the turn
    Defend { unit_id: String },
    /// Wait action - unit skips a turn
    Wait { unit_id: String },
}

/// Player struct - represents a player in a battle
/// - A player has an id
/// - A player has units
pub struct Player {
    pub id: String,
    pub units: Vec<Unit>,
}

impl Battle {
    pub fn new(grid: Box<dyn Grid<Option<Unit>>>, players: Vec<Player>) -> Self {
        Battle {
            grid,
            players,
            turns: Vec::new(),
            current_turn: 0,
            winner: None,
        }
    }

    /// Starts the battle by creating the first turn
    pub fn start(&mut self) {
        let mut units = Vec::new();
        for player in &self.players {
            for unit in &player.units {
                units.push(unit.clone());
            }
        }

        let actions_budget: u32 = 3;
        let usable_units = get_usable_units(&units, actions_budget);

        self.turns.push(Turn {
            player_id: self.players[0].id.clone(),
            actions_budget,
            actions: Vec::new(),
            finished: false,
            units,
            usable_units,
        });
    }
}
