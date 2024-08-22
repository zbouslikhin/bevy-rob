use bevy::app::{App, Plugin, Startup, Update};
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use camera::setup_camera;
use control::{movement, rotation};

pub mod camera;

mod control;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanOrbitCameraPlugin);
        app.add_systems(Startup, setup_camera);
        app.add_systems(Update, (movement, rotation));
    }
}
