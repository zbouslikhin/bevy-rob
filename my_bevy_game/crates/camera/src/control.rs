use bevy::prelude::*;
use std::f32::consts::*;

use bevy::{
    ecs::{query::With, system::Query},
    time::Time,
};

pub fn movement(
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

pub fn rotation(
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

pub fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in &mut query {
        transform.rotation = Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            time.elapsed_seconds() * PI / 5.0,
            -FRAC_PI_4,
        );
    }
}
