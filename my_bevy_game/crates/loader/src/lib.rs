use bevy::prelude::*;

pub struct AssetResolverPlugin;

impl Plugin for AssetResolverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(asset_server: Res<AssetServer>) {}
