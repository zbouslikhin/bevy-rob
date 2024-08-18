

const dgram = require('dgram');

// Create a socket
const client = dgram.createSocket('udp4');

// Message to send
const message = Buffer.from('Hello from Node.js');

// Send the message
client.send(message, 12345, '127.0.0.1', (err) => {
  if (err) {
    console.error('Error sending message:', err);
  } else {
    console.log('Message sent successfully');
  }
  client.close();
});
