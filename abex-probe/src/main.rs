mod data;
mod process;
mod ui;

use bevy::prelude::*;
use process::ProcessPlugin;
use ui::UIPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(UIPlugin)
        .add_plugin(ProcessPlugin)
        .run();
}
