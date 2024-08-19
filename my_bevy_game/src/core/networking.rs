use bevy::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{ReadableStream, ReadableStreamDefaultReader};
pub struct NetworkingPlugin;

#[wasm_bindgen]
pub struct StreamProcessor {
    reader: Option<ReadableStreamDefaultReader>,
}

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_stream);
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

    // Process the stream's chunks in WebAssembly
    pub async fn process_stream(&mut self) {
        if let Some(reader) = &self.reader {
            loop {
                let chunk_promise = reader.read();
                let chunk = wasm_bindgen_futures::JsFuture::from(chunk_promise)
                    .await
                    .unwrap();
                let chunk_value = js_sys::Reflect::get(&chunk, &"value".into()).unwrap();

                if chunk_value.is_undefined() {
                    break; // End of the stream
                }

                web_sys::console::log_1(&chunk_value); // Here you could process the chunk in WebAssembly (e.g., compute, transform)
            }
        }
    }
}

fn setup_stream() {
    // Placeholder for setting up the stream.
    // In a real app, this will be replaced by JavaScript-initiated stream handling
    spawn_local(async move {
        // Setup will be done from JavaScript
    });
}
