use aoe2_probe::Scenario;
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Button, ProgressBar},
    EguiContext,
};
use egui_extras::{Size, TableBuilder};

use crate::data::TriggerToken;

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
    mut commands: Commands,
    mut selected_trigger: ResMut<Vec<TriggerToken>>,
) {
    if ui_state.triggers {
        let listed_triggers: Vec<usize> = selected_trigger
            .iter()
            .map(|trigger_component| trigger_component.id)
            .collect();

        egui::Window::new("triggers-dialog")
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
                    .column(Size::initial(40.0).at_least(40.0))
                    .column(Size::initial(180.0).at_least(180.0))
                    .column(Size::initial(40.0).at_least(40.0))
                    .resizable(true)
                    .header(20.0, |mut header| {
                        header.col(|ui| {
                            ui.monospace("Row");
                        });
                        header.col(|ui| {
                            ui.monospace("Description");
                        });
                        header.col(|_| {});
                    })
                    .body(|mut body| {
                        for (index, trigger) in trigger_data.iter().enumerate() {
                            let trigger = trigger.try_map();
                            body.row(18.0, |mut row| {
                                row.col(|ui| {
                                    ui.monospace(index.to_string());
                                });
                                row.col(|ui| {
                                    ui.monospace(trigger["trigger_name"].try_str32().content());
                                });
                                row.col(|ui| {
                                    ui.set_enabled(!listed_triggers.contains(&index));
                                    if ui.button("Edit").clicked() {
                                        selected_trigger.push(TriggerToken {
                                            id: index,
                                            token: trigger.clone().into(),
                                        });
                                    }
                                });
                            })
                        }
                    });
            });
    }
}

pub fn trigger_dialog(
    mut content: ResMut<EguiContext>,
    ui_state: Res<UIState>,
    scenario: Res<Scenario>,
    mut selected_trigger: ResMut<Vec<TriggerToken>>,
) {
    for (index, trigger_component) in selected_trigger.iter_mut().enumerate() {
        if !ui_state.triggers {
            return;
        }
        egui::Window::new(format!("trigger-{}", index.to_string()))
            .title_bar(false)
            .resizable(false)
            .show(content.ctx_mut(), |ui| {
                let trigger = &mut trigger_component.token;
                egui::Grid::new("unique_id").show(ui, |ui| {
                    ui.vertical(|ui| {
                        let name = trigger.get_by_path_mut("trigger_name").try_mut_str32();
                        ui.label(name.content());
                        ui.separator();

                        ui.label("Name");
                        let mut name_str = String::from(name.content());
                        ui.text_edit_singleline(&mut name_str);
                        name.set_content(&name_str);

                        let description =
                            trigger.get_by_path_mut("short_description").try_mut_str32();
                        ui.label("Short description:");
                        let mut description_str = String::from(description.content());
                        ui.text_edit_singleline(&mut description_str);
                        description.set_content(&description_str);

                        let description = trigger
                            .get_by_path_mut("trigger_description")
                            .try_mut_str32();
                        ui.label("Trigger description:");
                        let mut description_str = String::from(description.content());
                        ui.text_edit_singleline(&mut description_str);
                        description.set_content(&description_str);

                        ui.horizontal(|ui| {
                            ui.label("Enable:");
                            let value = trigger.get_by_path_mut("enabled").try_mut_u32();
                            ui.selectable_value(value, 1, "True");
                            ui.selectable_value(value, 0, "False");
                        });

                        ui.horizontal(|ui| {
                            ui.label("Looping:");
                            let value = trigger.get_by_path_mut("looping").try_mut_i8();
                            ui.selectable_value(value, 1, "True");
                            ui.selectable_value(value, 0, "False");
                        });

                        ui.horizontal(|ui| {
                            ui.label("Description string table id:");
                            let table_id = trigger
                                .get_by_path_mut("description_string_table_id")
                                .try_mut_i32();
                            ui.add(egui::DragValue::new(table_id));
                        });

                        ui.horizontal(|ui| {
                            ui.label("Display as objective:");
                            let value =
                                trigger.get_by_path_mut("display_as_objective").try_mut_u8();
                            ui.selectable_value(value, 1, "True");
                            ui.selectable_value(value, 0, "False");
                        });

                        let display_as_objective =
                            *trigger.get_by_path_mut("display_as_objective").try_mut_u8();
                        ui.horizontal(|ui| {
                            ui.set_enabled(display_as_objective == 1);
                            ui.label("Objective description order:");
                            let table_id = trigger
                                .get_by_path_mut("objective_description_order")
                                .try_mut_u32();
                            ui.add(egui::DragValue::new(table_id));
                        });

                        ui.horizontal(|ui| {
                            ui.label("Make header:");
                            let value = trigger.get_by_path_mut("make_header").try_mut_u8();
                            ui.selectable_value(value, 1, "True");
                            ui.selectable_value(value, 0, "False");
                        });

                        ui.horizontal(|ui| {
                            ui.label("Short description string table id");
                            let value = trigger
                                .get_by_path_mut("short_description_string_table_id")
                                .try_mut_i32();
                            ui.add(egui::DragValue::new(value));
                        });

                        ui.horizontal(|ui| {
                            ui.label("Display on screen:");
                            let value = trigger.get_by_path_mut("display_on_screen").try_mut_u8();
                            ui.selectable_value(value, 1, "True");
                            ui.selectable_value(value, 0, "False");
                        });

                        ui.horizontal(|ui| {
                            ui.label("Mute objectives:");
                            let value = trigger.get_by_path_mut("mute_objectives").try_mut_u8();
                            ui.selectable_value(value, 1, "True");
                            ui.selectable_value(value, 0, "False");
                        });
                    });

                    ui.vertical(|ui| {
                        ui.label("Effects");
                        ui.separator();

                        ui.horizontal(|ui| {
                            let value = trigger.get_by_path("number_of_effects").try_i32();
                            ui.label(format!("Number of effects: {}", value));
                        });

                        let effect_data = trigger.get_by_path_mut("effect_data").try_mut_vec();

                        ui.push_id("effects_table", |ui| {
                            TableBuilder::new(ui)
                                .striped(true)
                                .cell_layout(egui::Layout::left_to_right())
                                .column(Size::initial(40.0).at_least(40.0))
                                .column(Size::initial(180.0).at_least(180.0))
                                .column(Size::initial(40.0).at_least(40.0))
                                .resizable(true)
                                .header(20.0, |mut header| {
                                    header.col(|ui| {
                                        ui.monospace("Row");
                                    });
                                    header.col(|ui| {
                                        ui.monospace("Description");
                                    });
                                    header.col(|_| {});
                                })
                                .body(|mut body| {
                                    for (index, effect) in effect_data.iter().enumerate() {
                                        let effect = effect.try_map();
                                        body.row(18.0, |mut row| {
                                            row.col(|ui| {
                                                ui.monospace(index.to_string());
                                            });
                                            row.col(|ui| {
                                                ui.monospace(
                                                    effect["effect_type"].try_i32().to_string(),
                                                );
                                            });
                                            row.col(|ui| if ui.button("Edit").clicked() {});
                                        })
                                    }
                                });
                        });
                    });

                    ui.vertical(|ui| {
                        ui.label("Conditions:");
                        ui.separator();

                        ui.horizontal(|ui| {
                            let value = trigger.get_by_path("number_of_conditions").try_i32();
                            ui.label(format!("Number of conditions: {}", value));
                        });

                        let condition_data =
                            trigger.get_by_path_mut("condition_data").try_mut_vec();

                        ui.push_id("conditions_table", |ui| {
                            TableBuilder::new(ui)
                                .striped(true)
                                .cell_layout(egui::Layout::left_to_right())
                                .column(Size::initial(40.0).at_least(40.0))
                                .column(Size::initial(180.0).at_least(180.0))
                                .column(Size::initial(40.0).at_least(40.0))
                                .resizable(true)
                                .header(20.0, |mut header| {
                                    header.col(|ui| {
                                        ui.monospace("Row");
                                    });
                                    header.col(|ui| {
                                        ui.monospace("Description");
                                    });
                                    header.col(|_| {});
                                })
                                .body(|mut body| {
                                    for (index, condition) in condition_data.iter().enumerate() {
                                        let condition = condition.try_map();
                                        body.row(18.0, |mut row| {
                                            row.col(|ui| {
                                                ui.monospace(index.to_string());
                                            });
                                            row.col(|ui| {
                                                ui.monospace(
                                                    condition["condition_type"]
                                                        .try_i32()
                                                        .to_string(),
                                                );
                                            });
                                            row.col(|ui| if ui.button("Edit").clicked() {});
                                        })
                                    }
                                });
                        });
                    });

                    ui.end_row();
                });
            });
    }
}
