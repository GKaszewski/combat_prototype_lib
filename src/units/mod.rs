use crate::common::Position;

#[derive(Clone, Debug, Default)]
pub struct Unit {
    pub id: String,
    pub name: String,
    pub health: i32,
    pub attack: i32,
    pub defense: i32,
    pub movement_points: i32,
    pub level: u32,
    pub experience: u32,
    pub range: i32,
    pub count: i32,
    pub initiative: i32,
}

pub struct UnitBuilder {
    id: String,
    name: String,
    health: i32,
    attack: i32,
    defense: i32,
    movement_points: i32,
    level: u32,
    experience: u32,
    range: i32,
    count: i32,
    initiative: i32,
}

impl UnitBuilder {
    pub fn new() -> Self {
        UnitBuilder {
            id: String::new(),
            name: String::new(),
            health: 0,
            attack: 0,
            defense: 0,
            movement_points: 0,
            level: 0,
            experience: 0,
            range: 0,
            count: 1,
            initiative: 1,
        }
    }

    pub fn id(mut self, id: &str) -> Self {
        self.id = id.to_string();
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn health(mut self, health: i32) -> Self {
        self.health = health;
        self
    }

    pub fn attack(mut self, attack: i32) -> Self {
        self.attack = attack;
        self
    }

    pub fn defense(mut self, defense: i32) -> Self {
        self.defense = defense;
        self
    }

    pub fn movement_points(mut self, movement_points: i32) -> Self {
        self.movement_points = movement_points;
        self
    }

    pub fn level(mut self, level: u32) -> Self {
        self.level = level;
        self
    }

    pub fn experience(mut self, experience: u32) -> Self {
        self.experience = experience;
        self
    }

    pub fn range(mut self, range: i32) -> Self {
        self.range = range;
        self
    }

    pub fn count(mut self, count: i32) -> Self {
        self.count = count;
        self
    }

    pub fn initiative(mut self, initiative: i32) -> Self {
        self.initiative = initiative;
        self
    }

    pub fn build(self) -> Unit {
        Unit {
            id: self.id,
            name: self.name,
            health: self.health,
            attack: self.attack,
            defense: self.defense,
            movement_points: self.movement_points,
            level: self.level,
            experience: self.experience,
            range: self.range,
            count: self.count,
            initiative: self.initiative,
        }
    }
}

impl Unit {
    pub fn new(
        id: &str,
        name: &str,
        health: i32,
        attack: i32,
        defense: i32,
        movement_points: i32,
        level: u32,
        experience: u32,
        range: i32,
        count: i32,
        initiative: i32,
    ) -> Unit {
        Unit {
            id: id.to_string(),
            name: name.to_string(),
            health,
            attack,
            defense,
            movement_points,
            level,
            experience,
            range,
            count,
            initiative,
        }
    }

    pub fn builder() -> UnitBuilder {
        UnitBuilder::new()
    }
}

pub struct UnitInBattle {
    pub unit: Unit,
    pub position: Position,
}

impl UnitInBattle {
    pub fn new(unit: Unit, position: Position) -> UnitInBattle {
        UnitInBattle { unit, position }
    }
}
