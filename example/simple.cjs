const { openWebpage } = require('@lhechenberger/open-webpage');

openWebpage({ title: 'Example', url: 'https://example.com' }).then(() =>
  console.log('Window was closed'),
);
