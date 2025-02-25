import { openWebpage } from '../js/async.mjs';

const controller = new AbortController();
const cancelSignal = controller.signal;

console.log('Opening webpage...');
const process = openWebpage({ url: 'https://example.com' }, { cancelSignal });

setTimeout(() => {
  controller.abort();
  console.log('Aborting process...');
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
