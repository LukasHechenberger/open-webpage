const { openWebpage } = require('../out/index.js');

openWebpage({ title: 'QRcard', url: 'https://example.com' }).then(() =>
  console.log('Window was closed'),
);
