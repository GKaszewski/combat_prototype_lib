use crate::units::Unit;

pub fn get_usable_units(units: &Vec<Unit>, actions_budget: u32) -> Vec<Unit> {
    let mut usable_units = units.clone();
    usable_units.sort_by(|unit1, unit2| unit1.initiative.cmp(&unit2.initiative));
    usable_units.reverse();

    usable_units
        .iter()
        .take(actions_budget as usize)
        .map(|unit| unit.clone())
        .collect()
}
