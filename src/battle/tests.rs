use crate::units::Unit;

use super::utils::get_usable_units;

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
