import { openWebpage } from '../js/async.mjs';

const controller = new AbortController();
const process = openWebpage({ url: 'https://example.com' }, { cancelSignal: controller.signal });

setTimeout(() => {
  console.log('Aborting process...');
  controller.abort();
}, [5000]);

try {
  await process;
} catch (error) {
  if (error.isCanceled) {
    console.log('Process was cancelled');
  } else {
    throw error;
  }
}
