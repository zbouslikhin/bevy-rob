import { JSDOM } from 'jsdom';
import { ReadableStream } from '@web-std/stream';

// Create a DOM environment to simulate a browser
const { window } = new JSDOM('', { url: 'http://localhost/' });

// Mock global objects that WebAssembly might expect
global.window = window;
global.document = window.document;
global.ReadableStream = ReadableStream;

// Dynamically import the wasm-bindgen module
async function main() {
  // Import the WebAssembly module
  const wasm = await import('../my_bevy_game/out/my_bevy_game.js');

  // Create a ReadableStream (e.g., emit numbers like in the browser example)
  const stream = new ReadableStream({
    start(controller) {
      let count = 0;
      const interval = setInterval(() => {
        console.log(`Pushing to stream: ${count} ${JSON.stringify({ "zaid": 1 })`);
        controller.enqueue(count++);

        if (count > 5) {
          clearInterval(interval);
          controller.close();
        }
      }, 1000);  // Emit one value per second
    }
  });

  // Create the StreamProcessor from the WebAssembly module
  const processor = new wasm.StreamProcessor(stream);

  // Process the stream in WebAssembly
  console.log("Processing stream in WebAssembly...");
  await processor.process_stream();
}

main().catch(err => {
  console.error("Error occurred:", err);
});

