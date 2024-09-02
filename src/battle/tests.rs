use crate::units::Unit;

use super::{attack, utils::get_usable_units};

#[test]
fn test_get_usable_units() {
    let actions_budget = 3;
    let units = vec![
        Unit {
            id: "1".to_string(),
            initiative: 5,
            ..Default::default()
        },
        Unit {
            id: "2".to_string(),
            initiative: 4,
            ..Default::default()
        },
        Unit {
            id: "3".to_string(),
            initiative: 1,
            ..Default::default()
        },
        Unit {
            id: "4".to_string(),
            initiative: 1,
            ..Default::default()
        },
        Unit {
            id: "5".to_string(),
            initiative: 1,
            ..Default::default()
        },
        Unit {
            id: "6".to_string(),
            initiative: 1,
            ..Default::default()
        },
    ];

    let usable_units = get_usable_units(&units, actions_budget);

    assert_eq!(usable_units.len(), 3);
    assert_eq!(usable_units[0].id, "1");
    assert_eq!(usable_units[1].id, "2");
    assert_eq!(usable_units[2].id, "6");

    let actions_budget = 2;
    let usable_units = get_usable_units(&units, actions_budget);

    assert_eq!(usable_units.len(), 2);
    assert_eq!(usable_units[0].id, "1");
    assert_eq!(usable_units[1].id, "2");

    let actions_budget = 1;
    let usable_units = get_usable_units(&units, actions_budget);

    assert_eq!(usable_units.len(), 1);
    assert_eq!(usable_units[0].id, "1");

    let actions_budget = 6;
    let usable_units = get_usable_units(&units, actions_budget);

    assert_eq!(usable_units.len(), 6);
    assert_eq!(usable_units[0].id, "1");
    assert_eq!(usable_units[1].id, "2");
    assert_eq!(usable_units[2].id, "6");
    assert_eq!(usable_units[3].id, "5");
    assert_eq!(usable_units[4].id, "4");
    assert_eq!(usable_units[5].id, "3");
}

#[test]
fn test_attack() {
    let mut attacker = Unit {
        id: "1".to_string(),
        name: "Archer".to_string(),
        health: 10,
        attack: 5,
        defense: 2,
        count: 1,
        ..Default::default()
    };

    let mut defender = Unit {
        id: "2".to_string(),
        name: "Swordsman".to_string(),
        health: 10,
        attack: 3,
        defense: 3,
        count: 1,
        ..Default::default()
    };

    attack(&mut attacker, &mut defender);

    assert_eq!(attacker.count, 1);
    assert_eq!(attacker.health, 7);
    assert_eq!(defender.count, 1);
    assert_eq!(defender.health, 4);

    attack(&mut attacker, &mut defender);

    assert_eq!(attacker.count, 1);
    assert_eq!(attacker.health, 7);
    assert_eq!(defender.count, 0);
    assert_eq!(defender.health, 0);
}
