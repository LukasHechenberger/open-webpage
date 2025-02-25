import { openWebpage } from '../js/async.mjs';

await openWebpage({ title: 'QRcard', url: 'https://example.com' });
console.log('Window was closed');
