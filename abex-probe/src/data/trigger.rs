use std::collections::HashMap;

use aoe2_probe::{parse::Token, Scenario};
use bevy::prelude::{EventReader, ResMut};

#[derive(Clone)]
pub struct Record {
    pub token: Token,
}

pub struct Unselect {
    pub index: usize,
}

pub fn unselect_trigger(
    mut event_reader: EventReader<Unselect>,
    mut selected_trigger: ResMut<HashMap<usize, Record>>,
) {
    for event in event_reader.iter() {
        selected_trigger.remove(&event.index);
    }
}

pub struct WriteBack {
    pub index: usize,
}

pub fn write_back_trigger(
    mut event_reader: EventReader<WriteBack>,
    selected_trigger: ResMut<HashMap<usize, Record>>,
    mut scenario: ResMut<Scenario>,
) {
    for event in event_reader.iter() {
        let trigger_token = selected_trigger[&event.index].clone();
        let trigger_data = scenario
            .versio
            .get_by_path_mut("/triggers/trigger_data")
            .try_mut_vec();

        trigger_data[event.index] = trigger_token.token;
    }
}

pub struct Save {
    pub index: usize,
}

pub fn save_trigger(
    mut event_reader: EventReader<Save>,
    mut selected_trigger: ResMut<HashMap<usize, Record>>,
    mut scenario: ResMut<Scenario>,
) {
    for event in event_reader.iter() {
        let trigger_token = selected_trigger[&event.index].clone();
        let trigger_data = scenario
            .versio
            .get_by_path_mut("/triggers/trigger_data")
            .try_mut_vec();

        trigger_data[event.index] = trigger_token.token;

        selected_trigger.remove(&event.index);
    }
}
