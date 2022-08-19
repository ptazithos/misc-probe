use std::collections::HashMap;

use aoe2_probe::{ConditionTweak, EffectTweak, Scenario};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Label},
    EguiContext,
};
use bevy_fluent::{Content, Localization};
use egui_extras::{Size, TableBuilder};

use crate::data::{condition, effect, trigger};

use super::UIState;

pub fn trigger_dialog(
    mut content: ResMut<EguiContext>,
    ui_state: Res<UIState>,
    scenario: Res<Scenario>,
    mut selected_triggers: ResMut<HashMap<usize, trigger::Record>>,
    mut selected_effects: ResMut<HashMap<(usize, usize), effect::Record>>,
    mut selected_conditions: ResMut<HashMap<(usize, usize), condition::Record>>,
    mut ev_unselect: EventWriter<trigger::Unselect>,
    mut ev_write_back: EventWriter<trigger::WriteBack>,
    mut ev_save: EventWriter<trigger::Save>,
    localization: Res<Localization>,
) {
    for (&index, trigger_component) in selected_triggers.iter_mut() {
        if !ui_state.triggers {
            return;
        }

        let mut apply_save_cancel_enable = true;
        egui::Window::new(format!("trigger-{}", index.to_string()))
            .title_bar(false)
            .resizable(false)
            .show(content.ctx_mut(), |ui| {
                let trigger = &mut trigger_component.token;
                egui::Grid::new(format!("trigger-grid-{}", index.to_string())).show(ui, |ui| {
                    ui.vertical(|ui| {
                        let name = trigger.get_by_path_mut("trigger_name").try_mut_str32();
                        ui.label(name.content());
                        ui.separator();

                        ui.label(localization.content("Name").unwrap());
                        let mut name_str = String::from(name.content());
                        ui.text_edit_singleline(&mut name_str);
                        name.set_content(&name_str);

                        let description =
                            trigger.get_by_path_mut("short_description").try_mut_str32();
                        ui.label(format!(
                            "{}:",
                            localization.content("Short-Description").unwrap()
                        ));
                        let mut description_str = String::from(description.content());
                        ui.text_edit_singleline(&mut description_str);
                        description.set_content(&description_str);

                        let description = trigger
                            .get_by_path_mut("trigger_description")
                            .try_mut_str32();
                        ui.label(format!(
                            "{}:",
                            localization.content("Trigger-Description").unwrap()
                        ));
                        let mut description_str = String::from(description.content());
                        ui.text_edit_singleline(&mut description_str);
                        description.set_content(&description_str);

                        ui.horizontal(|ui| {
                            ui.label(format!("{}:", localization.content("Enable").unwrap()));
                            let value = trigger.get_by_path_mut("enabled").try_mut_u32();
                            ui.selectable_value(value, 1, "True");
                            ui.selectable_value(value, 0, "False");
                        });

                        ui.horizontal(|ui| {
                            ui.label(format!("{}:", localization.content("Looping").unwrap()));
                            let value = trigger.get_by_path_mut("looping").try_mut_i8();
                            ui.selectable_value(value, 1, "True");
                            ui.selectable_value(value, 0, "False");
                        });

                        ui.horizontal(|ui| {
                            ui.label(format!(
                                "{}:",
                                localization.content("Description-String-Table-ID").unwrap()
                            ));
                            let table_id = trigger
                                .get_by_path_mut("description_string_table_id")
                                .try_mut_i32();
                            ui.add(egui::DragValue::new(table_id));
                        });

                        ui.horizontal(|ui| {
                            ui.label(format!(
                                "{}:",
                                localization.content("Display-As-Object").unwrap()
                            ));
                            let value =
                                trigger.get_by_path_mut("display_as_objective").try_mut_u8();
                            ui.selectable_value(value, 1, "True");
                            ui.selectable_value(value, 0, "False");
                        });

                        let display_as_objective =
                            *trigger.get_by_path_mut("display_as_objective").try_mut_u8();
                        ui.horizontal(|ui| {
                            ui.set_enabled(display_as_objective == 1);
                            ui.label(format!(
                                "{}:",
                                localization.content("Objective-Description-Order").unwrap()
                            ));
                            let table_id = trigger
                                .get_by_path_mut("objective_description_order")
                                .try_mut_u32();
                            ui.add(egui::DragValue::new(table_id));
                        });

                        ui.horizontal(|ui| {
                            ui.label(format!("{}:", localization.content("Make-Header").unwrap()));
                            let value = trigger.get_by_path_mut("make_header").try_mut_u8();
                            ui.selectable_value(value, 1, "True");
                            ui.selectable_value(value, 0, "False");
                        });

                        ui.horizontal(|ui| {
                            ui.label(format!(
                                "{}:",
                                localization
                                    .content("Short-Description-String-Table-ID")
                                    .unwrap()
                            ));
                            let value = trigger
                                .get_by_path_mut("short_description_string_table_id")
                                .try_mut_i32();
                            ui.add(egui::DragValue::new(value));
                        });

                        ui.horizontal(|ui| {
                            ui.label(format!(
                                "{}:",
                                localization.content("Display-On-Screen").unwrap()
                            ));
                            let value = trigger.get_by_path_mut("display_on_screen").try_mut_u8();
                            ui.selectable_value(value, 1, "True");
                            ui.selectable_value(value, 0, "False");
                        });

                        ui.horizontal(|ui| {
                            ui.label(format!(
                                "{}:",
                                localization.content("Mute-Objectives").unwrap()
                            ));
                            let value = trigger.get_by_path_mut("mute_objectives").try_mut_u8();
                            ui.selectable_value(value, 1, "True");
                            ui.selectable_value(value, 0, "False");
                        });
                    });

                    ui.vertical(|ui| {
                        ui.label(localization.content("Effects").unwrap());
                        ui.separator();

                        ui.horizontal(|ui| {
                            let value = trigger.get_by_path("number_of_effects").try_i32();
                            ui.label(format!(
                                "{}: {}",
                                localization.content("Number-Of-Effects").unwrap(),
                                value
                            ));
                        });

                        let effect_data = trigger.get_by_path_mut("effect_data").try_mut_vec();

                        ui.push_id(format!("effect-table-{}", index.to_string()), |ui| {
                            TableBuilder::new(ui)
                                .striped(true)
                                .cell_layout(egui::Layout::left_to_right())
                                .column(Size::initial(20.0).at_least(20.0))
                                .column(Size::initial(300.0).at_least(300.0))
                                .column(Size::initial(40.0).at_least(40.0))
                                .header(20.0, |mut header| {
                                    header.col(|ui| {
                                        ui.monospace(localization.content("Row").unwrap());
                                    });
                                    header.col(|ui| {
                                        ui.monospace(localization.content("Description").unwrap());
                                    });
                                    header.col(|_| {});
                                })
                                .body(|mut body| {
                                    for (effect_index, effect) in effect_data.iter().enumerate() {
                                        let res =
                                            EffectTweak::translate(&scenario, effect).unwrap();
                                        let height = (((res.0.len() + res.1.len()) as f32) / 50.0)
                                            .ceil()
                                            * 18.0;

                                        body.row(height, |mut row| {
                                            row.col(|ui| {
                                                ui.monospace(effect_index.to_string());
                                            });

                                            row.col(|ui| {
                                                ui.horizontal_top(|ui| {
                                                    ui.horizontal_wrapped(|ui| {
                                                        ui.label(res.0);
                                                        ui.add(
                                                            Label::new(
                                                                egui::RichText::new(res.1).weak(),
                                                            )
                                                            .wrap(true),
                                                        );
                                                    });
                                                });
                                            });
                                            row.col(|ui| {
                                                let selected = selected_effects
                                                    .contains_key(&(index, effect_index));
                                                apply_save_cancel_enable =
                                                    apply_save_cancel_enable && !selected;
                                                ui.set_enabled(!selected);
                                                if ui
                                                    .button(localization.content("Edit").unwrap())
                                                    .clicked()
                                                {
                                                    selected_effects.insert(
                                                        (index, effect_index),
                                                        effect::Record {
                                                            token: effect.clone(),
                                                        },
                                                    );
                                                }
                                            });
                                        })
                                    }
                                });
                        });
                    });

                    ui.vertical(|ui| {
                        ui.label(localization.content("Conditions").unwrap());
                        ui.separator();

                        ui.horizontal(|ui| {
                            let value = trigger.get_by_path("number_of_conditions").try_i32();
                            ui.label(format!(
                                "{}: {}",
                                localization.content("Number-Of-Conditions").unwrap(),
                                value
                            ));
                        });

                        let condition_data =
                            trigger.get_by_path_mut("condition_data").try_mut_vec();

                        ui.push_id(format!("condition-table-{}", index.to_string()), |ui| {
                            TableBuilder::new(ui)
                                .striped(true)
                                .cell_layout(egui::Layout::left_to_right())
                                .column(Size::initial(20.0).at_least(20.0))
                                .column(Size::initial(240.0).at_least(240.0))
                                .column(Size::initial(40.0).at_least(40.0))
                                .header(20.0, |mut header| {
                                    header.col(|ui| {
                                        ui.monospace(localization.content("Row").unwrap());
                                    });
                                    header.col(|ui| {
                                        ui.monospace(localization.content("Description").unwrap());
                                    });
                                    header.col(|_| {});
                                })
                                .body(|mut body| {
                                    for (condition_index, condition) in
                                        condition_data.iter().enumerate()
                                    {
                                        let res = ConditionTweak::translate(&scenario, condition)
                                            .unwrap();

                                        let height = (((res.0.len() + res.1.len()) as f32) / 45.0)
                                            .ceil()
                                            * 18.0;

                                        body.row(height, |mut row| {
                                            row.col(|ui| {
                                                ui.monospace(condition_index.to_string());
                                            });

                                            row.col(|ui| {
                                                ui.horizontal_top(|ui| {
                                                    ui.horizontal_wrapped(|ui| {
                                                        ui.label(res.0);
                                                        ui.add(
                                                            Label::new(
                                                                egui::RichText::new(res.1).weak(),
                                                            )
                                                            .wrap(true),
                                                        );
                                                    });
                                                });
                                            });
                                            row.col(|ui| {
                                                let selected = selected_conditions
                                                    .contains_key(&(index, condition_index));
                                                apply_save_cancel_enable =
                                                    apply_save_cancel_enable && !selected;
                                                ui.set_enabled(!selected);
                                                if ui
                                                    .button(localization.content("Edit").unwrap())
                                                    .clicked()
                                                {
                                                    selected_conditions.insert(
                                                        (index, condition_index),
                                                        condition::Record {
                                                            token: condition.clone(),
                                                        },
                                                    );
                                                }
                                            });
                                        })
                                    }
                                });
                        });
                    });
                    ui.end_row();
                });

                ui.separator();

                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(), |ui| {
                        ui.set_enabled(apply_save_cancel_enable);
                        if ui.button("Save").clicked() {
                            ev_save.send(trigger::Save { index });
                        };
                        if ui.button("Apply").clicked() {
                            ev_write_back.send(trigger::WriteBack { index });
                        };
                        if ui.button("Cancel").clicked() {
                            ev_unselect.send(trigger::Unselect { index });
                        };
                    })
                });
            });
    }
}
