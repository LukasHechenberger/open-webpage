import { openWebpage } from '@lhechenberger/open-webpage';

await openWebpage({ title: 'Example', url: 'https://example.com' });
console.log('Window was closed');
