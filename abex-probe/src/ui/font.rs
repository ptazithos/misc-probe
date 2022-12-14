use bevy::prelude::ResMut;
use bevy_egui::{egui, EguiContext};

pub fn setup_font(mut content: ResMut<EguiContext>) {
    let mut fonts = egui::FontDefinitions::default();

    let mut font = egui::FontData::from_static(include_bytes!("../../resource/zpix.ttf"));
    font.tweak.scale = 0.85;
    fonts.font_data.insert("zpix".to_owned(), font);

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .push("zpix".to_owned());

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("zpix".to_owned());

    content.ctx_mut().set_fonts(fonts);
}
