use bevy::prelude::*;

use bevy::{
    app::{App, Plugin, Startup, Update},
    ecs::{
        query::With,
        system::{Commands, Query},
    },
    time::Time,
};

pub struct RobotPlugin;

impl Plugin for RobotPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, (rotation, movement));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SceneBundle {
        scene: asset_server.load("http://localhost:3000/models/base.gltf#Scene0"),
        ..default()
    });
    commands.spawn(SceneBundle {
        scene: asset_server.load("http://localhost:3000/models/link1.gltf#Scene0"),
        ..default()
    });
    commands.spawn(SceneBundle {
        scene: asset_server.load("http://localhost:3000/models/link2.gltf#Scene0"),
        ..default()
    });
    commands.spawn(SceneBundle {
        scene: asset_server.load("http://localhost:3000/models/link3.gltf#Scene0"),
        ..default()
    });
    commands.spawn(SceneBundle {
        scene: asset_server.load("http://localhost:3000/models/link4.gltf#Scene0"),
        ..default()
    });
    commands.spawn(SceneBundle {
        scene: asset_server.load("http://localhost:3000/models/link5.gltf#Scene0"),
        ..default()
    });
    commands.spawn(SceneBundle {
        scene: asset_server.load("http://localhost:3000/models/link6.gltf#Scene0"),
        ..default()
    });
}

fn movement(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    // Calculate translation to move the cubes and ground plane
    let mut translation = Vec3::ZERO;

    // Horizontal forward and backward movement
    if input.pressed(KeyCode::KeyW) {
        translation.z += 100.0;
    } else if input.pressed(KeyCode::KeyS) {
        translation.z -= 100.0;
    }

    // Horizontal left and right movement
    if input.pressed(KeyCode::KeyA) {
        translation.x += 100.0;
    } else if input.pressed(KeyCode::KeyD) {
        translation.x -= 100.0;
    }

    // Vertical movement
    if input.pressed(KeyCode::ShiftLeft) {
        translation.y += 100.0;
    } else if input.pressed(KeyCode::Space) {
        translation.y -= 100.0;
    }

    translation *= 2.0 * time.delta_seconds();

    // Apply translation
    for mut transform in &mut query {
        transform.translation += translation;
    }
}

fn rotation(
    mut query: Query<&mut Transform, With<Camera>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut transform = query.single_mut();
    let delta = time.delta_seconds();

    if input.pressed(KeyCode::ArrowLeft) {
        transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(delta));
    } else if input.pressed(KeyCode::ArrowRight) {
        transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(-delta));
    }
}
