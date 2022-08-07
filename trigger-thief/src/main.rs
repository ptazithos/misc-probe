use std::{
    sync::mpsc::{self, Receiver},
    thread,
};

use aoe2_probe::Scenario;
use eframe::egui::{self, Button, ProgressBar};

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
    receiver: Option<Receiver<f64>>,
    progress: f32,
}

impl eframe::App for SinglePageApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.set_enabled(self.receiver.is_none());
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
                ui.set_enabled(self.receiver.is_none());
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
                if ui
                    .add_enabled(self.receiver.is_none(), Button::new("Move triggers"))
                    .clicked()
                {
                    let path_to_src = path_to_src.clone();
                    let path_to_dst = path_to_dst.clone();

                    let (tx, rx) = mpsc::channel();
                    self.receiver = Some(rx);

                    thread::spawn(move || {
                        let src = Scenario::from_file(&path_to_src);
                        let triggers = src.versio.get_by_path("/triggers/trigger_data").try_vec();
                        let len = triggers.len();
                        tx.send(0.25).unwrap();

                        let mut dst = Scenario::from_file(&path_to_dst);
                        let mut trigger_proxy = dst.triggers_proxy();
                        tx.send(0.5).unwrap();

                        for (index, trigger) in triggers.iter().enumerate() {
                            trigger_proxy.push(trigger.clone());
                            tx.send(0.5 + 0.5 * (index as f64 / len as f64)).unwrap();
                        }
                        dst.to_file("./temp.aoe2scenario");
                        tx.send(1.0).unwrap();
                    });
                }

                if let Some(rx) = &self.receiver {
                    match rx.try_recv() {
                        Ok(progress) => {
                            println!("{}", progress);
                            self.progress = progress as f32;
                            if progress == 1.0 {
                                self.receiver = None;
                                self.progress = 0.0;
                            }
                        }
                        Err(_) => {
                            println!("Not yet");
                        }
                    }
                    ui.label(format!("progress:{}%", self.progress));
                    ui.add(ProgressBar::new(self.progress));
                    ctx.request_repaint();
                }
            }
        });
    }
}
