use bevy::app::Plugin;
use bevy::app::{App, Startup};
use bevy::core_pipeline::core_3d::Camera3dBundle;
use bevy::ecs::component::Component;
use bevy::ecs::system::Commands;
use bevy::math::Vec3;
use bevy::transform::components::Transform;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanOrbitCameraPlugin);
        app.add_systems(Startup, setup);
    }
}

#[derive(Component)]
pub struct MyCameraMarker;

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 10.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        PanOrbitCamera {
            // Panning the camera changes the focus, and so you most likely want to disable
            // panning when setting the focus manually
            pan_sensitivity: 0.0,
            // If you want to fully control the camera's focus, set smoothness to 0 so it
            // immediately snaps to that location. If you want the 'follow' to be smoothed,
            // leave this at default or set it to something between 0 and 1.
            pan_smoothness: 0.0,
            ..Default::default()
        },
        MyCameraMarker,
    ));
}
