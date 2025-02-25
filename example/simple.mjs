import { openWebpage } from '../out/index.mjs';

await openWebpage({ title: 'QRcard', url: 'https://example.com' });
console.log('Window was closed');
