use aoe2_probe::Scenario;
use bevy::prelude::{Res, ResMut, State};
use bevy_egui::{
    egui::{self, Button, ProgressBar},
    EguiContext,
};
use egui_extras::{Size, TableBuilder};

use super::{ScenarioState, UIState};

pub fn file_dialog(
    mut content: ResMut<EguiContext>,
    mut ui_state: ResMut<UIState>,
    mut scenario_state: ResMut<State<ScenarioState>>,
) {
    if ui_state.file.show {
        egui::Window::new("File")
            .title_bar(false)
            .resizable(false)
            .show(content.ctx_mut(), |ui| {
                let is_loading = *scenario_state.current() == ScenarioState::Loading;
                let load_button_str = if is_loading { "Loading" } else { "Load" };

                ui.set_enabled(!is_loading);
                ui.horizontal(|ui| {
                    ui.label("Src scenario:");
                    if ui.button("Open file…").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file() {
                            ui_state.file.path_to_src = Some(path.display().to_string());
                        }
                    }
                });

                if let Some(picked_path) = &ui_state.file.path_to_src {
                    ui.horizontal(|ui| {
                        ui.label("Picked file:");
                        ui.monospace(picked_path);
                    });

                    ui.horizontal(|ui| {
                        if ui
                            .add_enabled(!is_loading, Button::new(load_button_str))
                            .clicked()
                        {
                            scenario_state.set(ScenarioState::Loading).unwrap();
                        }

                        if is_loading {
                            ui.add(ProgressBar::new(0.5));
                        }
                    });
                }
            });
    }
}

pub fn triggers_dialog(
    mut content: ResMut<EguiContext>,
    ui_state: Res<UIState>,
    scenario: Res<Scenario>,
) {
    if ui_state.triggers {
        egui::Window::new("Triggers")
            .title_bar(false)
            .resizable(false)
            .show(content.ctx_mut(), |ui| {
                let versio = &scenario.versio;

                let trigger_version = versio.get_by_path("/triggers/trigger_version").try_f64();
                ui.label(format!("Trigger version: {}", &trigger_version));

                let number_of_triggers =
                    versio.get_by_path("/triggers/number_of_triggers").try_u32();
                ui.label(format!("Number of triggers: {}", &number_of_triggers));

                let trigger_data = versio.get_by_path("/triggers/trigger_data").try_vec();
                ui.label("Trigger data:");
                TableBuilder::new(ui)
                    .striped(true)
                    .cell_layout(egui::Layout::left_to_right())
                    .column(Size::initial(60.0).at_least(40.0))
                    .column(Size::initial(180.0).at_least(120.0))
                    .column(Size::remainder().at_least(60.0))
                    .resizable(true)
                    .header(20.0, |mut header| {
                        header.col(|ui| {
                            ui.heading("Row");
                        });
                        header.col(|ui| {
                            ui.heading("Description");
                        });
                    })
                    .body(|mut body| {
                        for (index, trigger) in trigger_data.iter().enumerate() {
                            let trigger = trigger.try_map();
                            body.row(18.0, |mut row| {
                                row.col(|ui| {
                                    ui.label(index.to_string());
                                });
                                row.col(|ui| {
                                    ui.label(trigger["trigger_name"].try_str32().content());
                                });
                            })
                        }
                    });
            });
    }
}
