mod core;
mod plugins;
mod robots;

use bevy::app::App;
use bevy::prelude::*;
use core::camera::CameraPlugin;
use core::networking::NetworkingPlugin;
// use plugins::list_object::ObjectListPlugin;
use plugins::web_asset::WrappedWebAssetPlugin;
use robots::RobotPlugin;
use wasm_bindgen::prelude::*;

#[derive(Resource)]
struct MyResource {
    value: i32,
}

fn define_app() {
    App::new()
        .insert_resource(MyResource { value: 0 })
        .add_plugins(WrappedWebAssetPlugin)
        .add_plugins(NetworkingPlugin)
        .add_plugins(CameraPlugin)
        // .add_plugins(ObjectListPlugin)
        .add_plugins(RobotPlugin)
        .add_plugins(bevy_stl::StlPlugin)
        .add_systems(Startup, setup)
        .run();
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[wasm_bindgen]
pub fn start() {
    log("Hello from Rust!");
    define_app();
}

fn main() {
    define_app();
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
