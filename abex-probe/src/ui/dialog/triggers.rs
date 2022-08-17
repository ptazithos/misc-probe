use std::collections::HashMap;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use bevy_fluent::Localization;
use egui_extras::{Size, TableBuilder};

use super::UIState;
use crate::data::trigger;
use aoe2_probe::Scenario;

pub fn triggers_dialog(
    mut content: ResMut<EguiContext>,
    ui_state: Res<UIState>,
    scenario: Res<Scenario>,
    mut selected_trigger: ResMut<HashMap<usize, trigger::Record>>,
    localization: Res<Localization>,
) {
    if ui_state.triggers {
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
                                    ui.set_enabled(!selected_trigger.contains_key(&index));
                                    if ui.button("Edit").clicked() {
                                        selected_trigger.insert(
                                            index,
                                            trigger::Record {
                                                token: trigger.clone().into(),
                                            },
                                        );
                                    }
                                });
                            })
                        }
                    });
            });
    }
}
