mod plugins;

use bevy::app::App;
use bevy::ecs::system::ResMut;
use bevy::prelude::*;

use rob_camera::CameraPlugin;
use rob_lightning::lightning::LightningPlugin;
use rob_network::readable_stream::NetworkingPlugin;
use rob_spawner::robots::RobotPlugin;

// use plugins::list_object::ObjectListPlugin;
use plugins::web_asset::WrappedWebAssetPlugin;
use wasm_bindgen::prelude::*;

fn define_app() -> App {
    let mut app = App::new();

    // Chain all method calls, which modify the app in place
    app.add_plugins(WrappedWebAssetPlugin)
        .add_plugins(NetworkingPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(LightningPlugin)
        // .add_plugins(ObjectListPlugin)
        .add_plugins(RobotPlugin)
        .add_plugins(bevy_stl::StlPlugin)
        .add_systems(Startup, setup);

    app
}

#[wasm_bindgen]
pub fn start() {
    let mut app = define_app();
    app.run();
}

fn main() {
    let mut app = define_app();
    app.run();
}

#[derive(Component)]
pub struct Bullet {
    name: String,
    max_size: f32,
    min_size: f32,
    scale_factor: f32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::srgb(0.8, 0.7, 0.6)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // Plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(5.0, 5.0)),
        material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
        ..default()
    });
    commands.spawn((
        PbrBundle {
            mesh: asset_server.load("http://localhost:3000/everest.stl"),
            material: materials.add(Color::srgb(0.8, 0.7, 0.6)),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..Default::default()
        },
        Bullet {
            name: "hello".to_string(),
            max_size: 1.0,
            min_size: 0.1,
            scale_factor: 0.05,
        },
    ));
}
