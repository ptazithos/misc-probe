use std::collections::HashMap;

use aoe2_probe::{ConditionTweak, Scenario};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::{data::condition, ui::UIState};

pub fn condition_dialog(
    scenario: Res<Scenario>,
    mut content: ResMut<EguiContext>,
    ui_state: Res<UIState>,
    mut selected_conditions: ResMut<HashMap<(usize, usize), condition::Record>>,
    mut ev_unselect: EventWriter<condition::Unselect>,
    mut ev_write_back: EventWriter<condition::WriteBack>,
    mut ev_save: EventWriter<condition::Save>,
) {
    if !ui_state.triggers {
        return;
    }

    for (&index, condition_record) in selected_conditions.iter_mut() {
        egui::Window::new(format!("condition-{}-{}", index.0, index.1))
            .title_bar(false)
            .resizable(false)
            .show(content.ctx_mut(), |ui| {
                let condition = &mut condition_record.token;
                let condition_scheme =
                    ConditionTweak::scheme(scenario.version(), &condition).unwrap();

                ui.label("Effect");
                ui.separator();

                ui.label(format!("Type: {}", condition_scheme.name));
                for &attr in &condition_scheme.attrs {
                    match attr {
                        "inverted" => {
                            ui.horizontal(|ui| {
                                let ref_value = condition.get_by_path_mut(attr).try_mut_i32();
                                ui.label(attr);
                                ui.selectable_value(ref_value, 1, "True");
                                ui.selectable_value(ref_value, 0, "False");
                            });
                        }
                        "xs_function" => {
                            ui.horizontal(|ui| {
                                let ref_value = condition
                                    .get_by_path_mut(attr)
                                    .try_mut_str32()
                                    .content_mut();
                                ui.label(attr);
                                ui.text_edit_singleline(ref_value);
                            });
                        }
                        _ => {
                            ui.horizontal(|ui| {
                                let ref_value = condition.get_by_path_mut(attr).try_mut_i32();
                                ui.label(attr);
                                ui.add(egui::DragValue::new(ref_value));
                            });
                        }
                    }
                }
                ui.separator();
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(), |ui| {
                        if ui.button("Save").clicked() {
                            ev_save.send(condition::Save { index });
                        };
                        if ui.button("Apply").clicked() {
                            ev_write_back.send(condition::WriteBack { index });
                        };
                        if ui.button("Cancel").clicked() {
                            ev_unselect.send(condition::Unselect { index });
                        };
                    })
                });
            });
    }
}
