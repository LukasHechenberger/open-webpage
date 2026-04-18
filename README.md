# open-webpage

> Open a webpage in a custom window from nodejs

> [!NOTE]  
> This was basically a playground project for me to get to know napi-rs. I think it turned out pretty well :)

## Installation

```shell
bun add @lhechenberger/open-webpage
pnpm add @lhechenberger/open-webpage # or yarn add, npm install ...
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

```ts
import { openWebpage } from '@lhechenberger/open-webpage';

openWebpage(
  {
    url: 'https://your.url', // The URL to open
    title: 'My webpage', // Window title
    fullscreen: true, // Open in fullscreen
  },
  {
    // Options for execa, see https://www.npmjs.com/package/execa
  },
);
```

There are additional options available to customize the window's appearance, use an IDE to get hints.

## CLI

This package also ships with a small command line application

```shell
npx @lhechenberger/open-webpage --help
```
