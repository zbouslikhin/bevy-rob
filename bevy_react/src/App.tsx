import { useEffect, useState } from 'react';
import { ReadableStream } from '@web-std/stream';
import init, { StreamProcessor } from '../../my_bevy_game/out/my_bevy_game.js'; // Ensure this path is correct
import reactLogo from './assets/react.svg';
import viteLogo from '/vite.svg';
import './App.css';

function App() {
  const [count, setCount] = useState(0);
  const [streamProcessor, setStreamProcessor] = useState(null); // State to store the StreamProcessor

  useEffect(() => {
    // Initialize WebAssembly module and setup stream

    init().then(() => {

    });
  }, []);



  const setParam = () => {

    try {

      // Create a ReadableStream
      const stream = new ReadableStream({
        start(controller) {
          let count = 0;
          const interval = setInterval(() => {
            console.log(`Pushing to stream: ${count}`);
            controller.enqueue(count++);
            controller.enqueue(
              JSON.stringify({ zaid: 1 }))

            if (count > 5) {
              clearInterval(interval);
              controller.close();
            }
          }, 1000); // Emit one value per second
        }
      });

      // Create the StreamProcessor from the WebAssembly module
      const processor = new StreamProcessor(stream);
      setStreamProcessor(processor);

      console.log("Processing stream in WebAssembly...");
      processor.process_stream().then(() => {

      });

    } catch (err) {
      console.error('Error initializing WASM module or processing stream:', err);
    }
  }

  return (
    <>
      <div>
        <a href="https://vitejs.dev" target="_blank">
          <img src={viteLogo} className="logo" alt="Vite logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <h1>Vite + React</h1>
      <div className="card">
        <button onClick={setParam}>
          count is {count}
        </button>
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
      </div>
      <p className="read-the-docs">
        Click on the Vite and React logos to learn more
      </p>
    </>
  )
}

export default App
