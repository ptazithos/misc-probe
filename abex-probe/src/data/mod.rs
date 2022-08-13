use aoe2_probe::parse::Token;
use bevy::prelude::{Plugin, ResMut, SystemSet};

use crate::ui::ScenarioState;

pub struct TriggerToken {
    pub id: usize,
    pub token: Token,
}

pub struct DataPlugin;

impl Plugin for DataPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(Vec::<TriggerToken>::new())
            .add_system_set(SystemSet::on_enter(ScenarioState::Loading).with_system(clear_cache));
    }
}

fn clear_cache(mut selected_trigger: ResMut<Vec<TriggerToken>>) {
    selected_trigger.clear();
}
