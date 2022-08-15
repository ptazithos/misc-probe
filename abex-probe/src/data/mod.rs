pub mod condition;
pub mod effect;
pub mod trigger;

use crate::ui::LoadState;
use bevy::prelude::{Plugin, ResMut, SystemSet};
use condition::{save_condition, unselect_condition, write_back_condition};
use effect::{unselect_effect, write_back_effect};
use std::collections::HashMap;
use trigger::{save_trigger, unselect_trigger, write_back_trigger};

use self::effect::save_effect;

pub struct DataPlugin;

impl Plugin for DataPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<trigger::Unselect>()
            .add_event::<trigger::WriteBack>()
            .add_event::<trigger::Save>()
            .add_event::<effect::Unselect>()
            .add_event::<effect::WriteBack>()
            .add_event::<effect::Save>()
            .add_event::<condition::Unselect>()
            .add_event::<condition::WriteBack>()
            .add_event::<condition::Save>()
            .insert_resource(HashMap::<usize, trigger::Record>::new())
            .insert_resource(HashMap::<(usize, usize), effect::Record>::new())
            .insert_resource(HashMap::<(usize, usize), condition::Record>::new())
            .add_system_set(SystemSet::on_enter(LoadState::Loading).with_system(clear_cache))
            .add_system_set(SystemSet::on_update(LoadState::Loaded).with_system(unselect_trigger))
            .add_system_set(SystemSet::on_update(LoadState::Loaded).with_system(write_back_trigger))
            .add_system_set(SystemSet::on_update(LoadState::Loaded).with_system(save_trigger))
            .add_system_set(SystemSet::on_update(LoadState::Loaded).with_system(unselect_effect))
            .add_system_set(SystemSet::on_update(LoadState::Loaded).with_system(write_back_effect))
            .add_system_set(SystemSet::on_update(LoadState::Loaded).with_system(save_effect))
            .add_system_set(SystemSet::on_update(LoadState::Loaded).with_system(unselect_condition))
            .add_system_set(
                SystemSet::on_update(LoadState::Loaded).with_system(write_back_condition),
            )
            .add_system_set(SystemSet::on_update(LoadState::Loaded).with_system(save_condition));
    }
}

fn clear_cache(
    mut selected_trigger: ResMut<HashMap<usize, trigger::Record>>,
    mut selected_effects: ResMut<HashMap<(usize, usize), effect::Record>>,
) {
    selected_trigger.clear();
    selected_effects.clear();
}
