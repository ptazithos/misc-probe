mod data;
mod process;
mod ui;

use bevy::prelude::*;
use data::DataPlugin;
use process::ProcessPlugin;
use ui::UIPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(DataPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(ProcessPlugin)
        .run();
}
