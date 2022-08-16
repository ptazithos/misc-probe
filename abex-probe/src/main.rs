mod data;
mod process;
mod ui;

use bevy::prelude::*;
use data::DataPlugin;
use process::ProcessPlugin;
use ui::UIPlugin;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Abex Probe".to_string(),
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(DataPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(ProcessPlugin)
        .run();
}
