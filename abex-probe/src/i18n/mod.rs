use bevy::{asset::LoadState, prelude::*};
use bevy_fluent::{AssetServerExt, BundleAsset, FluentPlugin, LocalizationBuilder};

pub struct I18nPlugin;

impl Plugin for I18nPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let current_locale = locale_config::Locale::current().to_string();
        app.add_plugin(FluentPlugin)
            .insert_resource(
                bevy_fluent::Locale::new(current_locale.parse().unwrap())
                    .with_default("en-US".parse().unwrap()),
            )
            .add_state(I18nAssetStatus::Loading)
            .add_system_set(SystemSet::on_enter(I18nAssetStatus::Loading).with_system(setup))
            .add_system_set(SystemSet::on_update(I18nAssetStatus::Loading).with_system(load));
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum I18nAssetStatus {
    Loading,
    Ready,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handles = asset_server
        .load_glob::<BundleAsset>("locales/**/main.ftl.ron")
        .unwrap();
    commands.insert_resource(handles);
}

fn load(
    mut commands: Commands,
    localization_builder: LocalizationBuilder,
    asset_server: Res<AssetServer>,
    mut asset_state: ResMut<State<I18nAssetStatus>>,
    handles: Res<Vec<Handle<BundleAsset>>>,
) {
    if let LoadState::Loaded =
        asset_server.get_group_load_state(handles.iter().map(|handle| handle.id))
    {
        let localization = localization_builder.build(&*handles);
        commands.remove_resource::<Vec<Handle<BundleAsset>>>();
        commands.insert_resource(localization);
        asset_state.set(I18nAssetStatus::Ready).unwrap();
    }
}
