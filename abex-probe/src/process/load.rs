use std::thread;

use aoe2_probe::Scenario;
use bevy::prelude::{Commands, Res, ResMut, State};
use crossbeam_channel::{bounded, Receiver};

use crate::ui::{ScenarioState, UIState};

pub fn load_scenario(mut commands: Commands, ui_state: Res<UIState>) {
    if let Some(picked_path) = &ui_state.file.path_to_src {
        let path_to_src = picked_path.to_string();
        let (tx, rx) = bounded::<Result<Scenario, String>>(1);

        #[allow(unused_must_use)]
        thread::spawn(move || {
            let res = Scenario::from_file(&path_to_src);
            tx.send(res);
        });

        commands.insert_resource(rx);
    }
}

pub fn watch_scenario(
    receiver: ResMut<Receiver<Result<Scenario, String>>>,
    mut commands: Commands,
    mut scenario_state: ResMut<State<ScenarioState>>,
) {
    if !receiver.is_empty() {
        let result = receiver.recv().unwrap();
        match result {
            Ok(scenario) => {
                commands.insert_resource(scenario);
                scenario_state.set(ScenarioState::Loaded).unwrap();
            }
            Err(_) => {
                scenario_state.set(ScenarioState::NotYet).unwrap();
            }
        }
    }
}
