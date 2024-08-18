const cors = require('cors');
const express = require('express');
const morgan = require('morgan');
const path = require('path');

const app = express();
const PORT = process.env.PORT || 3000;

// Use morgan to log requests
app.use(morgan('combined'));
app.use(cors({
  origin: 'http://localhost:5173' // allow requests from Vite
}));

// Serve static files from the 'public' directory
app.use(express.static(path.join(__dirname, 'public')));

// Start the server
app.listen(PORT, () => {
  console.log(`CDN server is running on http://localhost:${PORT}`);
});
