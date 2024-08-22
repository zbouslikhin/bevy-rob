use bevy::prelude::*;
use bevy_extern_events::{queue_event, ExternEventsPlugin};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{ReadableStream, ReadableStreamDefaultReader};

pub struct NetworkingPlugin;

#[wasm_bindgen]
pub struct StreamProcessor {
    reader: Option<ReadableStreamDefaultReader>,
}

#[derive(Default)]
pub struct MyEvent(pub String);

#[derive(Resource, Reflect, Default)]
pub struct MyEventResource(i32);

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MyEventResource>();
        app.add_plugins(ExternEventsPlugin::<MyEvent>::default());
        // app.add_systems(Update, handle_event_system);
    }
}

#[wasm_bindgen]
impl StreamProcessor {
    #[wasm_bindgen(constructor)]
    pub fn new(stream: ReadableStream) -> StreamProcessor {
        let reader = stream
            .get_reader()
            .dyn_into::<ReadableStreamDefaultReader>()
            .unwrap();
        StreamProcessor {
            reader: Some(reader),
        }
    }

    pub async fn process_stream(&mut self) {
        if let Some(reader) = &self.reader {
            loop {
                let chunk_promise = reader.read();
                let chunk = wasm_bindgen_futures::JsFuture::from(chunk_promise)
                    .await
                    .unwrap();
                let chunk_value = js_sys::Reflect::get(&chunk, &"value".into()).unwrap();

                if chunk_value.is_undefined() {
                    break;
                }

                let chunk_value_str = chunk_value.as_string().unwrap();

                queue_event(MyEvent { 0: chunk_value_str });
            }
        }
    }
}

// fn handle_event_system(
//     mut event_reader: EventReader<ExternEvent<MyEvent>>,
//     mut param_set: ParamSet<(
//         Query<(&Handle<Mesh>, &Transform), With<Bullet>>, // Query to get the mesh handle and Transform
//         Query<&mut PanOrbitCamera, With<MyCameraMarker>>, // Query to get the camera's PanOrbitCamera component
//     )>,
//     meshes: Res<Assets<Mesh>>, // Resource to access meshes
// ) {
//     for _event in event_reader.read() {
//         let data = _event.0 .0.clone();
//         let w = data.as_str();
//         let v: serde_json::Value = serde_json::from_str(w).unwrap();
//         for key in v.as_object().unwrap().keys() {
//             info!("{}", key);
//         }
//         info!("Received in deserialized {:}", v["zaid"]);
//         // Handle your event here
//         if let Ok((mesh_handle, transform)) = param_set.p0().get_single() {
//             info!("Inside mesh");
//             if let Some(mesh) = meshes.get(mesh_handle) {
//                 // Calculate the center of the mesh in world space
//                 let mesh_center = calculate_mesh_center(mesh, transform);
//
//                 // Safely update the camera's target focus to this calculated center
//                 if let Ok(mut camera) = param_set.p1().get_single_mut() {
//                     println!(
//                         "Resetting camera to focus on mesh center at: {:?}",
//                         mesh_center
//                     );
//
//                     camera.target_focus = mesh_center;
//                     camera.force_update = true;
//                 }
//             } else {
//                 println!("Mesh not found!");
//             }
//         } else {
//             println!("Static entity not found!");
//         }
//     }
// }
//
// fn calculate_mesh_center(mesh: &Mesh, transform: &Transform) -> Vec3 {
//     let positions = match mesh.attribute(Mesh::ATTRIBUTE_POSITION) {
//         Some(VertexAttributeValues::Float32x3(positions)) => positions,
//         _ => panic!("Expected Float32x3 vertex positions"),
//     };
//
//     let mut min = Vec3::splat(f32::MAX);
//     let mut max = Vec3::splat(f32::MIN);
//
//     for &position in positions {
//         let pos = transform.transform_point(Vec3::from(position));
//         min = min.min(pos);
//         max = max.max(pos);
//     }
//
//     (min + max) / 2.0
// }
