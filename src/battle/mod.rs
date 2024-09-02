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
    pub state: BattleState,
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

/// Battle state enum - represents the state of a battle
/// - Started - the battle has started
/// - StartedTurn - the turn has started with the given current turn number
/// - TurnInProgress - the turn is in progress
/// - TurnFinished - the turn has finished
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BattleState {
    Started,
    StartedTurn(u32),
    TurnInProgress,
    TurnFinished,
    Finished,
}

impl Battle {
    pub fn new(grid: Box<dyn Grid<Option<Unit>>>, players: Vec<Player>) -> Self {
        Battle {
            grid,
            players,
            turns: Vec::new(),
            current_turn: 0,
            winner: None,
            state: BattleState::Started,
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

        self.current_turn = 0;
        self.state = BattleState::StartedTurn(0);
    }
}

/// Calculates the damage done by an attacker to a defender
/// Returns the damage done
fn calculate_damage(attacker: &Unit, defender: &Unit) -> i32 {
    let attack = attacker.attack;
    let defense = defender.defense;

    let base_damage = attack;
    let damage_modifier = 1.0 + 0.05 * (attack as f32 - defense as f32);

    let total_damage =
        (base_damage as f32 * attacker.count as f32 * damage_modifier).round() as i32;

    total_damage
}

/// Applies damage to a unit
/// Returns the number of casualties
fn apply_damage(unit: &mut Unit, damage: i32) -> i32 {
    let mut remainig_damage = damage;
    let mut casualties = 0;
    let base_health = unit.health;

    while remainig_damage > 0 && unit.count > 0 {
        if unit.health > remainig_damage {
            unit.health -= remainig_damage;
            remainig_damage = 0;
        } else {
            remainig_damage -= unit.health;
            unit.count -= 1;
            if unit.count > 0 {
                unit.health = base_health;
            }
            casualties += 1;
        }
    }

    casualties
}

/// Attacks a defender with an attacker
/// Applies damage to both units
/// If the attacker is dead, it is removed from the grid
/// If the defender is dead, it is removed from the grid
pub fn attack(attacker: &mut Unit, defender: &mut Unit) {
    let damage = calculate_damage(attacker, defender);
    let _ = apply_damage(defender, damage);

    if defender.count > 0 {
        let damage = calculate_damage(defender, attacker);
        let _ = apply_damage(attacker, damage);

        if attacker.count == 0 {
            attacker.health = 0;
            // attacker is dead, remove it from the grid
            println!("Attacker is dead");
        }
    } else {
        defender.health = 0;
        // defender is dead, remove it from the grid
        println!("Defender is dead");
    }
}
