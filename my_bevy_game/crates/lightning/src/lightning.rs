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

pub struct LightningPlugin;

impl Plugin for LightningPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DirectionalLightShadowMap { size: 4096 });
        app.add_systems(Startup, setup);
        app.add_systems(Update, animate_light_direction);
    }
}

fn setup(mut commands: Commands) {
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
