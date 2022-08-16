use super::{LoadState, UIState};
use aoe2_probe::{ExportFormat, Scenario};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self},
    EguiContext,
};

pub fn export_dialog(
    mut content: ResMut<EguiContext>,
    mut ui_state: ResMut<UIState>,
    scenario_state: ResMut<State<LoadState>>,
    scenario: Res<Scenario>,
) {
    if ui_state.export {
        egui::Window::new("Export")
            .title_bar(false)
            .resizable(false)
            .show(content.ctx_mut(), |ui| {
                if *scenario_state.current() == LoadState::Loaded {
                    ui.horizontal(|ui| {
                        ui.label("Dst Scenario:");
                        if ui.button("Select file...").clicked() {
                            if let Some(path) = rfd::FileDialog::new().save_file() {
                                ui_state.file.path_to_dst = Some(path.display().to_string());
                            }
                        }
                    });

                    if let Some(mut picked_path) = ui_state.file.path_to_dst.clone() {
                        ui.horizontal(|ui| {
                            ui.label("Export format:");
                            ui.selectable_value(
                                &mut ui_state.file.export_format,
                                ExportFormat::AoE2Scenario,
                                "AOE2Scenario",
                            );
                            ui.selectable_value(
                                &mut ui_state.file.export_format,
                                ExportFormat::JSON,
                                "JSON",
                            );
                        });
                        match ui_state.file.export_format {
                            ExportFormat::AoE2Scenario => picked_path.push_str(".aoe2scenario"),
                            ExportFormat::JSON => picked_path.push_str(".json"),
                        }

                        ui.horizontal(|ui| {
                            ui.label("Output file:");
                            ui.monospace(&picked_path);
                        });
                        if ui.button("Export").clicked() {
                            scenario
                                .to_file(&picked_path, ui_state.file.export_format.clone())
                                .unwrap();
                        }
                    }
                }
            });
    }
}
