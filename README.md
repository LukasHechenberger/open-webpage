# open-webpage

> Open a webpage in a custom window from nodejs

## Installation

```shell
ppnm add @lhechenberger/open-webpage # or yarn add, npm install ...
```

## Usage

**Basic example**

```js
import { openWebpage } from '@lhechenberger/open-webpage';

// Promise resolves once the window is closed
await openWebpage({ url: 'https://example.com' });
console.log('Window was closed');
```

...which results in a simple native window popping up, looking like this:

![A simple example](docs/assets/screenshot-simple-example.png)

**You can also use an abort controller to close the page manually**

```js
import { openWebpage } from '@lhechenberger/open-webpage';

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
```

**Additional options**

There are additional options available to customize the window's appearance, use an IDE to get hints.
