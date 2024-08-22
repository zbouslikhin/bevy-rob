mod constants;

use bevy::{
    prelude::*,
    render::{mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology},
};
use bevy_mod_outline::{OutlineBundle, OutlinePlugin, OutlineVolume};
use bevy_mod_picking::{DefaultPickingPlugins, PickableBundle};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

pub enum CubeFaceSegment {
    Center,
    CornerTopLeft,
    CornerTopRight,
    CornerBottomLeft,
    CornerBottomRight,
    EdgeTop,
    EdgeBottom,
    EdgeLeft,
    EdgeRight,
}
pub enum ViewCubeSide {
    Top,
    Bottom,
    Left,
    Right,
    Front,
    Back,
}

// fn segment_transform(segment: &ViewCubeSegment) -> (Vec3, Quat, Vec3) {
    match segment {
        ViewCubeSegment::Front => (
            Vec3::new(0.0, 0.5, 0.0),
            Quat::from_euler(
                EulerRot::XYZ,
                -90.0f32.to_radians(),
                0.0f32.to_radians(),
                0.0f32.to_radians(),
            ),
            Vec3::new(0.8, 0.8, 1.0),
        ),
        ViewCubeSegment::Back => (
            Vec3::new(0.0, -0.5, 0.0),
            Quat::from_euler(
                EulerRot::XYZ,
                90.0f32.to_radians(),
                0.0f32.to_radians(),
                180.0f32.to_radians(),
            ),
            Vec3::new(0.8, 0.8, 1.0),
        ),
        ViewCubeSegment::Right => (
            Vec3::new(0.5, 0.0, 0.0),
            Quat::from_euler(
                EulerRot::XYZ,
                0.0f32.to_radians(),
                90.0f32.to_radians(),
                -90.0f32.to_radians(),
            ),
            Vec3::new(0.8, 0.8, 1.0),
        ),
        ViewCubeSegment::Left => (
            Vec3::new(-0.5, 0.0, 0.0),
            Quat::from_euler(
                EulerRot::XYZ,
                0.0f32.to_radians(),
                -90.0f32.to_radians(),
                90.0f32.to_radians(),
            ),
            Vec3::new(0.8, 0.8, 1.0),
        ),
        ViewCubeSegment::Top => (
            Vec3::new(0.0, 0.0, 0.5),
            Quat::from_euler(EulerRot::XYZ, 0., 0., 0.),
            Vec3::new(0.8, 0.8, 1.0),
        ),
        ViewCubeSegment::Bottom => (
            Vec3::new(0.0, 0.0, -0.5),
            Quat::from_euler(
                EulerRot::XYZ,
                180.0f32.to_radians(),
                0.,
                180.0f32.to_radians(),
            ),
            Vec3::new(0.8, 0.8, 1.0),
        ),
        // Define transformations for other segments as needed
        _ => (Vec3::ZERO, Quat::IDENTITY, Vec3::ZERO),
    }
}

fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(1.0, 1.0, 1.0)))
        .add_plugins(PanOrbitCameraPlugin)
        .add_plugins(bevy_editor_pls::EditorPlugin::default())
        .add_plugins(DefaultPickingPlugins)
        .add_plugins(OutlinePlugin)
        .add_systems(Startup, setup_cam)
        .add_systems(Startup, spawn_viewcube)
        .run();
}

fn setup_cam(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(3.0, 3.0, 3.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        PanOrbitCamera {
            pan_sensitivity: 0.0,
            pan_smoothness: 0.0,
            ..Default::default()
        },
    ));
}

fn spawn_viewcube(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let faces = vec![
        (ViewCubeSide::Top, Vec3::new(0.0, 0.0, 0.5), Quat::IDENTITY),
        (ViewCubeSide::Bottom, Vec3::new(0.0, 0.0, -0.5), Quat::from_rotation_x(180.0f32.to_radians())),
        (ViewCubeSide::Front, Vec3::new(0.0, 0.5, 0.0), Quat::from_rotation_x(-90.0f32.to_radians())),
        (ViewCubeSide::Back, Vec3::new(0.0, -0.5, 0.0), Quat::from_rotation_x(90.0f32.to_radians())),
        (ViewCubeSide::Left, Vec3::new(-0.5, 0.0, 0.0), Quat::from_rotation_y(-90.0f32.to_radians())),
        (ViewCubeSide::Right, Vec3::new(0.5, 0.0, 0.0), Quat::from_rotation_y(90.0f32.to_radians())),
    ];

    for face in &faces {
        let (translation, rotation, scale) = segment_transform(segment);


        let material_handle = materials.add(StandardMaterial {
            base_color_texture: Some(texture_handle.clone()),
            cull_mode: None,
            ..Default::default()
        });

        commands.spawn((
            PbrBundle {
                mesh,
                material: material_handle,
                transform: Transform {
                    translation,
                    rotation,
                    scale: Vec3::new(0.8, 0.8, 1.0),  // Adjust scale if needed
                    ..Default::default()
                },
                ..Default::default()
            },
            PickableBundle {
                interaction: bevy_mod_picking::focus::PickingInteraction::None,
                ..Default::default()
            },
            OutlineBundle {
                outline: OutlineVolume {
                    visible: false,
                    width: 15.,
                    colour: Color::srgb(1.0, 0.84, 0.0),
                },
                ..Default::default()
            },
        ));
    }
}

fn spawn_segments_for_face(
    face: &str,
    translation: Vec3,
    rotation: Quat,
) {
    // For each face, spawn its segments
    let segments = vec![
        (ViewCubeSegment::Center, Vec3::ZERO),
        (ViewCubeSegment::CornerTopLeft, Vec3::new(-RECTANGLE_WIDTH, RECTANGLE_HEIGHT, 0.0)),
        (ViewCubeSegment::CornerTopRight, Vec3::new(RECTANGLE_WIDTH, RECTANGLE_HEIGHT, 0.0)),
        (ViewCubeSegment::CornerBottomLeft, Vec3::new(-RECTANGLE_WIDTH, -RECTANGLE_HEIGHT, 0.0)),
        (ViewCubeSegment::CornerBottomRight, Vec3::new(RECTANGLE_WIDTH, -RECTANGLE_HEIGHT, 0.0)),
        (ViewCubeSegment::EdgeTop, Vec3::new(0.0, RECTANGLE_HEIGHT, 0.0)),
        (ViewCubeSegment::EdgeBottom, Vec3::new(0.0, -RECTANGLE_HEIGHT, 0.0)),
        (ViewCubeSegment::EdgeLeft, Vec3::new(-RECTANGLE_WIDTH, 0.0, 0.0)),
        (ViewCubeSegment::EdgeRight, Vec3::new(RECTANGLE_WIDTH, 0.0, 0.0)),
    ];

    for (segment, offset) in segments {
        spawn_segment(face, segment, translation, rotation, offset);
    }
}

fn spawn_segment(
    face: &str,
    segment: ViewCubeSegment,
    base_translation: Vec3,
    rotation: Quat,
    offset: Vec3,
) {
    let texture_path = format!("{}.png", face);
    let texture_handle = asset_server.load(texture_path.as_str());

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle),
        cull_mode: None,
        ..Default::default()
    });

    let translation = base_translation + rotation * offset;  // Apply rotation to the offset

    
}