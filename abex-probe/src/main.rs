mod data;
mod i18n;
mod process;
mod ui;

use bevy::{asset::AssetServerSettings, prelude::*};
use data::DataPlugin;
use i18n::I18nPlugin;
use process::ProcessPlugin;
use ui::UIPlugin;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Abex Probe".to_string(),
            ..default()
        })
        .insert_resource(AssetServerSettings {
            asset_folder: "assets".to_string(),
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(I18nPlugin)
        .add_plugin(DataPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(ProcessPlugin)
        .run();
}
