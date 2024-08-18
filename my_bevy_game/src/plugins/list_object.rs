use bevy::prelude::*;
use bevy::render::mesh::VertexAttributeValues;
use bevy_panorbit_camera::PanOrbitCamera;

use crate::core::camera::MyCameraMarker;
use crate::Bullet;

pub struct ObjectListPlugin;

impl Plugin for ObjectListPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, update_orange_text);
        app.add_systems(Update, handle_button_interactions);
        // app.add_systems(Update, update_camera_position);
    }
}

#[derive(Component)]
struct Count;

#[derive(Component)]
struct ObjectName(String);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("http://localhost:3000/Arial Unicode.ttf");

    commands
        .spawn(NodeBundle {
            style: Style {
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        margin: UiRect {
                            left: Val::Px(5.0),
                            right: Val::Px(5.0),
                            top: Val::Px(5.0),
                            bottom: Val::Px(5.0),
                        },
                        ..Default::default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    border_radius: BorderRadius::MAX,
                    background_color: Color::srgb(0.15, 0.15, 0.15).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        TextBundle::from_section(
                            "",
                            TextStyle {
                                font: font.clone(),
                                font_size: 20.0,
                                color: Color::WHITE,
                            },
                        ),
                        Count,
                        ObjectName("B".to_string()),
                    ));
                });
        });
}

fn update_orange_text(
    query: Query<((&Transform, &Bullet), &Handle<Mesh>)>,
    mut orange_count: Query<&mut Text, With<Count>>,
) {
    for n in query.iter() {
        let mut oranges_text = orange_count.single_mut();
        oranges_text.sections[0].value = n.0 .1.name.clone();
    }
}

fn handle_button_interactions(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut param_set: ParamSet<(
        Query<(&Handle<Mesh>, &Transform), With<Bullet>>, // Query to get the mesh handle and Transform
        Query<&mut PanOrbitCamera, With<MyCameraMarker>>, // Query to get the camera's PanOrbitCamera component
    )>,
    meshes: Res<Assets<Mesh>>, // Resource to access meshes
    mut ev_d: EventReader<UdpMessageReceived>,
) {
    for event in ev_d.read() {
        println!("from list_object {}: {}", event.src, event.data);
        if let Ok((mesh_handle, transform)) = param_set.p0().get_single() {
            if let Some(mesh) = meshes.get(mesh_handle) {
                // Calculate the center of the mesh in world space
                let mesh_center = calculate_mesh_center(mesh, transform);

                // Safely update the camera's target focus to this calculated center
                if let Ok(mut camera) = param_set.p1().get_single_mut() {
                    println!(
                        "Resetting camera to focus on mesh center at: {:?}",
                        mesh_center
                    );

                    camera.target_focus = mesh_center;
                    camera.force_update = true;
                }
            } else {
                println!("Mesh not found!");
            }
        }
    }

    for interaction in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                println!("Pressed");

                // Retrieve the static entity's mesh and Transform
                if let Ok((mesh_handle, transform)) = param_set.p0().get_single() {
                    if let Some(mesh) = meshes.get(mesh_handle) {
                        // Calculate the center of the mesh in world space
                        let mesh_center = calculate_mesh_center(mesh, transform);

                        // Safely update the camera's target focus to this calculated center
                        if let Ok(mut camera) = param_set.p1().get_single_mut() {
                            println!(
                                "Resetting camera to focus on mesh center at: {:?}",
                                mesh_center
                            );

                            camera.target_focus = mesh_center;
                            camera.force_update = true;
                        }
                    } else {
                        println!("Mesh not found!");
                    }
                } else {
                    println!("Static entity not found!");
                }
            }
            Interaction::Hovered => {
                println!("Hovered");
            }
            Interaction::None => {}
        }
    }
}
// Function to calculate the center of a mesh
fn calculate_mesh_center(mesh: &Mesh, transform: &Transform) -> Vec3 {
    let positions = match mesh.attribute(Mesh::ATTRIBUTE_POSITION) {
        Some(VertexAttributeValues::Float32x3(positions)) => positions,
        _ => panic!("Expected Float32x3 vertex positions"),
    };

    let mut min = Vec3::splat(f32::MAX);
    let mut max = Vec3::splat(f32::MIN);

    for &position in positions {
        let pos = transform.transform_point(Vec3::from(position));
        min = min.min(pos);
        max = max.max(pos);
    }

    (min + max) / 2.0
} //     println!("{:?}", name.0.name);
  //     parent
  //         .spawn(ButtonBundle {
  //             style: Style {
  //                 margin: UiRect {
  //                     left: Val::Px(5.0),
  //                     right: Val::Px(5.0),
  //                     top: Val::Px(5.0),
  //                     bottom: Val::Px(5.0),
  //                 },
  //                 ..Default::default()
  //             },
  //             ..Default::default()
  //         })
  //         .with_children(|parent| {
  //             parent.spawn(TextBundle::from_section(
  //                 name.0.name.clone(),
  //                 TextStyle {
  //                     font: font.clone(),
  //                     font_size: 20.0,
  //                     color: Color::WHITE,
  //                 },
  //             ));
  //         })
  //         .insert(ObjectName(name.0.name.clone()));
  // }
  //
