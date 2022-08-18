use super::{LoadState, UIState};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Button, ProgressBar},
    EguiContext,
};
use bevy_fluent::{Content, Localization};

pub fn file_dialog(
    mut content: ResMut<EguiContext>,
    mut ui_state: ResMut<UIState>,
    mut scenario_state: ResMut<State<LoadState>>,
    localization: Res<Localization>,
) {
    if ui_state.file.show {
        egui::Window::new("File")
            .title_bar(false)
            .resizable(false)
            .show(content.ctx_mut(), |ui| {
                let is_loading = *scenario_state.current() == LoadState::Loading;
                let load_button_str = if is_loading {
                    localization.content("Loading").unwrap()
                } else {
                    localization.content("Load").unwrap()
                };

                ui.set_enabled(!is_loading);
                ui.horizontal(|ui| {
                    ui.label(format!(
                        "{}:",
                        localization.content("Src-Scenario").unwrap()
                    ));
                    if ui
                        .button(format!("{}...", localization.content("Open-File").unwrap()))
                        .clicked()
                    {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("AoE2 Scenario", &["aoe2scenario"])
                            .pick_file()
                        {
                            ui_state.file.path_to_src = Some(path.display().to_string());
                        }
                    }
                });

                if let Some(picked_path) = &ui_state.file.path_to_src {
                    ui.horizontal(|ui| {
                        ui.label(format!("{}:", localization.content("Picked-File").unwrap()));
                        ui.monospace(picked_path);
                    });

                    ui.horizontal(|ui| {
                        if ui
                            .add_enabled(!is_loading, Button::new(load_button_str))
                            .clicked()
                        {
                            scenario_state.set(LoadState::Loading).unwrap();
                        }

                        if is_loading {
                            ui.add(ProgressBar::new(0.5));
                        }
                    });
                }
            });
    }
}
