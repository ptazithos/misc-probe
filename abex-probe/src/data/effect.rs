use std::collections::HashMap;

use aoe2_probe::parse::Token;
use bevy::prelude::*;

use super::trigger;

#[derive(Clone)]
pub struct Record {
    pub token: Token,
}

pub struct Unselect {
    pub index: (usize, usize),
}

pub fn unselect_effect(
    mut event_reader: EventReader<Unselect>,
    mut selected_effects: ResMut<HashMap<(usize, usize), Record>>,
) {
    for event in event_reader.iter() {
        selected_effects.remove(&event.index);
    }
}

pub struct WriteBack {
    pub index: (usize, usize),
}

pub fn write_back_effect(
    mut event_reader: EventReader<WriteBack>,
    selected_effects: ResMut<HashMap<(usize, usize), Record>>,
    mut selected_triggers: ResMut<HashMap<usize, trigger::Record>>,
) {
    for event in event_reader.iter() {
        let index = &event.index;
        let effect = selected_effects[index].token.clone();

        let trigger = &mut selected_triggers.get_mut(&index.0).unwrap().token;
        let effect_data = trigger.get_by_path_mut("effect_data").try_mut_vec();
        effect_data[index.1] = effect;
    }
}

pub struct Save {
    pub index: (usize, usize),
}

pub fn save_effect(
    mut event_reader: EventReader<Save>,
    mut selected_effects: ResMut<HashMap<(usize, usize), Record>>,
    mut selected_triggers: ResMut<HashMap<usize, trigger::Record>>,
) {
    for event in event_reader.iter() {
        let index = &event.index;
        let effect = selected_effects[index].token.clone();

        let trigger = &mut selected_triggers.get_mut(&index.0).unwrap().token;
        let effect_data = trigger.get_by_path_mut("effect_data").try_mut_vec();
        effect_data[index.1] = effect;

        selected_effects.remove(&event.index);
    }
}
