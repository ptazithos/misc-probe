use bevy::prelude::ResMut;

use super::UIState;

pub fn enter_loaded(mut ui_state: ResMut<UIState>) {
    let config = ui_state.as_mut();
    config.file.show = false;
    config.triggers = true;
    config.map = true;
    config.units = true;
}

pub fn enter_not_yet(mut ui_state: ResMut<UIState>) {
    let config = ui_state.as_mut();

    config.file.show = true;
    config.file.path_to_src = None;

    config.triggers = false;
    config.map = false;
    config.units = false;
}
