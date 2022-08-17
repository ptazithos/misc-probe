use bevy::prelude::{Res, ResMut, State};
use bevy_egui::{egui, EguiContext};
use bevy_fluent::{Content, Localization};

use super::{LoadState, UIState};

pub fn menu_bar(
    mut content: ResMut<EguiContext>,
    mut ui_state: ResMut<UIState>,
    scenario_state: Res<State<LoadState>>,
    localization: Res<Localization>,
) {
    egui::TopBottomPanel::top("menu_bar").show(content.ctx_mut(), |ui| {
        let loaded = LoadState::Loaded == *scenario_state.current();

        ui.horizontal(|ui| {
            ui.checkbox(
                &mut ui_state.file.show,
                localization.content("File").unwrap(),
            );
            ui.add_enabled_ui(loaded, |ui| {
                ui.checkbox(
                    &mut ui_state.export,
                    localization.content("Export").unwrap(),
                );
                ui.separator();
                ui.checkbox(
                    &mut ui_state.triggers,
                    localization.content("Triggers").unwrap(),
                );
                ui.checkbox(&mut ui_state.units, localization.content("Units").unwrap());
                ui.checkbox(&mut ui_state.map, localization.content("Map").unwrap());
            });
        });
    });
}
