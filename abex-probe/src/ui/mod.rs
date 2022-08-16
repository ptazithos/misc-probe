mod dialog;
mod font;
mod menu_bar;
mod watch;

use aoe2_probe::ExportFormat;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use dialog::file_dialog;
use menu_bar::menu_bar;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(EguiPlugin)
            .add_startup_system(font::setup_font)
            .insert_resource(UIState::default())
            .add_state(LoadState::NotYet)
            .add_system(menu_bar)
            .add_system(file_dialog.after(menu_bar))
            .add_system_set(
                SystemSet::on_enter(LoadState::NotYet).with_system(watch::enter_not_yet),
            )
            .add_system_set(
                SystemSet::on_update(LoadState::Loaded)
                    .with_system(watch::enter_loaded)
                    .after(menu_bar)
                    .with_system(dialog::export_dialog)
                    .with_system(dialog::condition_dialog)
                    .with_system(dialog::effect_dialog)
                    .with_system(dialog::trigger_dialog)
                    .with_system(dialog::triggers_dialog),
            );
    }
}

#[derive(Default)]
pub struct UIState {
    pub file: FileState,
    pub triggers: bool,
    pub units: bool,
    pub map: bool,
    pub export: bool,
}

pub struct FileState {
    pub show: bool,
    pub export_format: ExportFormat,
    pub path_to_src: Option<String>,
    pub path_to_dst: Option<String>,
}

impl Default for FileState {
    fn default() -> Self {
        Self {
            show: true,
            export_format: ExportFormat::AoE2Scenario,
            path_to_src: Default::default(),
            path_to_dst: Default::default(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum LoadState {
    NotYet,
    Loading,
    Loaded,
}
