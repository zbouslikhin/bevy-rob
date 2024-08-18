import init, { Runner } from '../../my_bevy_game/out/my_bevy_game.js'
import { useEffect, useRef, useState } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'

function App() {
  const [count, setCount] = useState(0)
  const ref = useRef()

  useEffect(() => {
    init().then(() => {
      console.log("Run")
      ref.current = new Runner();
      // start()
    }).catch((err) => {
      console.log(`an error occured ${err}`)
      // ref.current = new Runner();
      console.error(err)
    })
  }, [])

  const handleSendMessage = async () => {
    try {
      const res = await fetch('http://localhost:3001/send-message', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({
          message: message,
          port: 12345, // Replace with the target port
          address: '127.0.0.1', // Replace with the target address
        }),
      });

      const result = await res.text();
      setResponse(result);
    } catch (error) {
      console.error('Error sending message:', error);
      setResponse('Error sending message');
    }
  };

  const setParam = () => {
    handleSendMessage()
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
