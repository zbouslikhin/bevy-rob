use bevy::pbr::{CascadeShadowConfigBuilder, DirectionalLightShadowMap};
use bevy::prelude::*;
use std::f32::consts::*;

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
        app.insert_resource(DirectionalLightShadowMap { size: 4096 });
        app.add_systems(Startup, setup);
        app.add_systems(Update, (animate_light_direction, rotation, movement));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        // This is a relatively small scene, so use tighter shadow
        // cascade bounds than the default for better quality.
        // We also adjusted the shadow map to be larger since we're
        // only using a single cascade.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            num_cascades: 1,
            maximum_distance: 1.6,
            ..default()
        }
        .into(),
        ..default()
    });

    commands.spawn(SceneBundle {
        // scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/base.gltf")),
        scene: asset_server.load("http://localhost:3000/models/base.gltf#Scene0"),
        // scene: model_handle.clone(),
        ..default()
    });

    // commands.spawn(SceneBundle {
    //     scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/link1.gltf")),
    //     ..default()
    // });

    // commands.spawn(SceneBundle {
    //     scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/link2.gltf")),
    //     ..default()
    // });
    //
    // commands.spawn(SceneBundle {
    //     scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/link3.gltf")),
    //     ..default()
    // });
    //
    // commands.spawn(SceneBundle {
    //     scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/link4.gltf")),
    //     ..default()
    // });
    //
    // commands.spawn(SceneBundle {
    //     scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/link5.gltf")),
    //     ..default()
    // });
    //
    // commands.spawn(SceneBundle {
    //     scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/link6.gltf")),
    //     ..default()
    // });
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

fn animate_light_direction(
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
