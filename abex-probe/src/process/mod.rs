mod load;

use bevy::prelude::{Plugin, SystemSet};
use load::{load_scenario, watch_scenario};

use crate::ui::LoadState;

pub struct ProcessPlugin;

impl Plugin for ProcessPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(SystemSet::on_enter(LoadState::Loading).with_system(load_scenario))
            .add_system_set(SystemSet::on_update(LoadState::Loading).with_system(watch_scenario));
    }
}
