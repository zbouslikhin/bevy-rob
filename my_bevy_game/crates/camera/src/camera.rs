use bevy::core_pipeline::core_3d::Camera3dBundle;
use bevy::ecs::component::Component;
use bevy::ecs::system::Commands;
use bevy::math::Vec3;
use bevy::transform::components::Transform;
use bevy_panorbit_camera::PanOrbitCamera;

#[derive(Component)]
pub struct MyCameraMarker;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 10.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        PanOrbitCamera {
            pan_sensitivity: 0.0,
            pan_smoothness: 0.0,
            ..Default::default()
        },
        MyCameraMarker,
    ));
}
