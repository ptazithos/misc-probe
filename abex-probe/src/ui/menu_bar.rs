use bevy::prelude::{Res, ResMut, State};
use bevy_egui::{egui, EguiContext};

use super::{LoadState, UIState};

pub fn menu_bar(
    mut content: ResMut<EguiContext>,
    mut ui_state: ResMut<UIState>,
    scenario_state: Res<State<LoadState>>,
) {
    egui::TopBottomPanel::top("menu_bar").show(content.ctx_mut(), |ui| {
        let loaded = LoadState::Loaded == *scenario_state.current();

        ui.horizontal(|ui| {
            ui.checkbox(&mut ui_state.file.show, "File");
            ui.add_enabled_ui(loaded, |ui| {
                ui.checkbox(&mut ui_state.export, "Export");
                ui.separator();
                ui.checkbox(&mut ui_state.triggers, "Triggers");
                ui.checkbox(&mut ui_state.units, "Units");
                ui.checkbox(&mut ui_state.map, "Map");
            });
        });
    });
}
