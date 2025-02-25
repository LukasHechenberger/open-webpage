import { openWebpage } from '../js/async.mjs';

openWebpage({ title: 'QRcard', url: 'https://qrcardapp.com' }).then(
  () => console.log('Done'),
  console.error,
);
