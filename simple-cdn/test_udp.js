// Import the WebSocket module
const WebSocket = require('ws');

// Connect to the WebSocket server
const ws = new WebSocket('ws://127.0.0.1:8080');

// Handle connection open event
ws.on('open', function open() {
  console.log('Connected to the WebSocket server');

  // Send a message to the WebSocket server
  ws.send('Hello, Server!');
});

// Handle incoming messages
ws.on('message', function incoming(data) {
  console.log('Received from server: %s', data);
});

// Handle connection close event
ws.on('close', function close() {
  console.log('Disconnected from the WebSocket server');
});

// Handle errors
ws.on('error', function error(err) {
  console.error('WebSocket error:', err);
});

