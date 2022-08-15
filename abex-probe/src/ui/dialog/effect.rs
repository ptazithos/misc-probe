use std::collections::HashMap;

use aoe2_probe::{EffectTweak, Scenario};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

use crate::{data::effect, ui::UIState};

pub fn effect_dialog(
    scenario: Res<Scenario>,
    mut content: ResMut<EguiContext>,
    ui_state: Res<UIState>,
    mut selected_effects: ResMut<HashMap<(usize, usize), effect::Record>>,
    mut ev_unselect: EventWriter<effect::Unselect>,
    mut ev_write_back: EventWriter<effect::WriteBack>,
    mut ev_save: EventWriter<effect::Save>,
) {
    if !ui_state.triggers {
        return;
    }

    for (&index, effect_record) in selected_effects.iter_mut() {
        egui::Window::new(format!("effect-{}-{}", index.0, index.1))
            .title_bar(false)
            .resizable(false)
            .show(content.ctx_mut(), |ui| {
                let effect = &mut effect_record.token;
                let effect_scheme = EffectTweak::scheme(scenario.version(), &effect).unwrap();

                ui.label("Effect");
                ui.separator();

                ui.label(format!("Type: {}", effect_scheme.name));
                for &attr in &effect_scheme.attrs {
                    match attr {
                        "message" | "sound_name" => {
                            ui.horizontal(|ui| {
                                let ref_value =
                                    effect.get_by_path_mut(attr).try_mut_str32().content_mut();
                                ui.label(attr);
                                ui.text_edit_singleline(ref_value);
                            });
                        }
                        "selected_object_ids" => {
                            ui.horizontal(|ui| {
                                ui.label(attr);
                                let ids = effect.get_by_path_mut(attr).try_mut_vec();
                                for id_token in ids {
                                    ui.add(egui::DragValue::new(id_token.try_mut_i32()));
                                }
                            });
                        }
                        "operation" => {
                            ui.horizontal(|ui| {
                                let ref_value = effect.get_by_path_mut(attr).try_mut_i32();
                                ui.label(attr);
                                ui.selectable_value(ref_value, 1, "Set");
                                ui.selectable_value(ref_value, 2, "Add");
                                ui.selectable_value(ref_value, 3, "Subtract");
                                ui.selectable_value(ref_value, 4, "Multiply");
                                ui.selectable_value(ref_value, 5, "Divide");
                            });
                        }
                        "class" | "quantity" => {
                            let ref_value = effect.get_by_path_mut(attr).try_mut_i16();
                            ui.label(attr);
                            ui.add(egui::DragValue::new(ref_value));
                        }
                        _ => {
                            ui.horizontal(|ui| {
                                let ref_value = effect.get_by_path_mut(attr).try_mut_i32();
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
                            ev_save.send(effect::Save { index });
                        };
                        if ui.button("Apply").clicked() {
                            ev_write_back.send(effect::WriteBack { index });
                        };
                        if ui.button("Cancel").clicked() {
                            ev_unselect.send(effect::Unselect { index });
                        };
                    })
                });
            });
    }
}
