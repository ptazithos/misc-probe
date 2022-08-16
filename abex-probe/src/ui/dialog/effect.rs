use std::collections::HashMap;

use aoe2_probe::{prebuilt::ATTR_MAP, AttrTweak, EffectTweak, Scenario};
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
                let mut attrs = effect_scheme.attrs.clone();
                if effect_scheme.name == "MODIFY_ATTRIBUTE" {
                    let object_attributes_id = *effect.get_by_path("object_attributes").try_i32();
                    if object_attributes_id != 8 && object_attributes_id != 9 {
                        let index = attrs.iter().position(|&value| value == "class").unwrap();

                        attrs.remove(index);
                    }
                }

                for &attr in &attrs {
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
                            ui.horizontal(|ui| {
                                ui.label(attr);
                                ui.add(egui::DragValue::new(ref_value));
                            });
                        }
                        "object_attributes" => {
                            let attr_id = effect.get_by_path_mut(attr).try_mut_i32();
                            let content =
                                AttrTweak::translate(attr_id, scenario.version()).unwrap();

                            ui.horizontal(|ui| {
                                ui.label("Object attributes:");

                                egui::ComboBox::from_id_source(format!(
                                    "{}-{}-effect-{}",
                                    index.0, index.1, attr_id
                                ))
                                .selected_text(format!("{}", content))
                                .show_ui(ui, |ui| {
                                    ui.set_width(240.0);
                                    for (&key, &value) in ATTR_MAP.iter() {
                                        ui.selectable_value(attr_id, key, value);
                                    }
                                });
                            });
                        }
                        "trigger_id" => {
                            let ref_value = effect.get_by_path_mut(attr).try_mut_i32();
                            ui.label(attr);
                            let mut content = String::from("");

                            let triggers: Vec<(i32, &String)> = scenario
                                .versio
                                .get_by_path("triggers/trigger_data")
                                .try_vec()
                                .iter()
                                .enumerate()
                                .map(|(key, token)| {
                                    if key as i32 == *ref_value {
                                        content = token
                                            .get_by_path("trigger_name")
                                            .try_str32()
                                            .content()
                                            .clone();
                                    }
                                    (
                                        key as i32,
                                        token.get_by_path("trigger_name").try_str32().content(),
                                    )
                                })
                                .collect();

                            egui::ComboBox::from_id_source(format!(
                                "{}-{}-effect-{}",
                                index.0, index.1, attr
                            ))
                            .selected_text(format!("{}", content))
                            .show_ui(ui, |ui| {
                                ui.set_width(240.0);
                                for (key, value) in triggers.iter() {
                                    ui.selectable_value(ref_value, *key, value.clone());
                                }
                            });
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
