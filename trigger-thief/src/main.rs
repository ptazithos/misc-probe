use aoe2_probe::Scenario;
use eframe::egui;

fn main() {
    let options = eframe::NativeOptions {
        ..Default::default()
    };

    eframe::run_native(
        "Trigger Thief",
        options,
        Box::new(|_cc| Box::new(SinglePageApp::default())),
    );
}

#[derive(Default)]
struct SinglePageApp {
    path_to_src: Option<String>,
    path_to_dst: Option<String>,
}

impl eframe::App for SinglePageApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Src scenario:");
                if ui.button("Open file…").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.path_to_src = Some(path.display().to_string());
                    }
                }
            });

            if let Some(picked_path) = &self.path_to_src {
                ui.horizontal(|ui| {
                    ui.label("Picked file:");
                    ui.monospace(picked_path);
                });
            }

            ui.horizontal(|ui| {
                ui.label("Dst scenario:");
                if ui.button("Open file…").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_file() {
                        self.path_to_dst = Some(path.display().to_string());
                    }
                }
            });

            if let Some(picked_path) = &self.path_to_dst {
                ui.horizontal(|ui| {
                    ui.label("Picked file:");
                    ui.monospace(picked_path);
                });
            }

            if let (Some(path_to_src), Some(path_to_dst)) = (&self.path_to_src, &self.path_to_dst) {
                if ui.button("Move triggers").clicked() {
                    let src = Scenario::from_file(path_to_src);
                    let triggers = src.versio.get_by_path("/triggers/trigger_data").try_vec();

                    let mut dst = Scenario::from_file(path_to_dst);
                    let mut trigger_proxy = dst.triggers_proxy();

                    for trigger in triggers.iter() {
                        trigger_proxy.push(trigger.clone());
                    }

                    dst.to_file("./temp.aoe2scenario");
                }
            }
        });
    }
}
