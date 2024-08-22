use bevy::asset::LoadState;
use bevy::prelude::UntypedHandle;
use bevy::prelude::*;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetList>()
            .add_systems(Update, check_asset_loading);
    }
}

#[derive(Default, Resource)]
pub struct AssetList(pub Vec<UntypedHandle>);

pub fn check_asset_loading(
    asset_server: Res<AssetServer>,
    asset_list: Res<AssetList>,
    // mut next_state: ResMut<NextState<MainState>>,
) {
    match asset_server.get_load_state(asset_list.0.iter().map(|a| a.id())) {
        Some(LoadState::Loaded) => {
            next_state.set(MainState::Game);
        }
        LoadState::Failed => {
            error!("asset loading error");
        }
        _ => {}
    };
}
