use bevy::app::App;
use bevy::DefaultPlugins;
use bevy::{
    app::Plugin,
    asset::{AssetMode, AssetPlugin},
};
use bevy_web_asset::WebAssetPlugin;

pub struct WrappedWebAssetPlugin;

impl Plugin for WrappedWebAssetPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((WebAssetPlugin::default(), DefaultPlugins));
        if !app.is_plugin_added::<AssetPlugin>() {
            app.add_plugins(AssetPlugin {
                mode: AssetMode::Unprocessed,
                ..Default::default()
            });
        }
    }
}
