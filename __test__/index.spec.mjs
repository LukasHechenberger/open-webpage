import { test } from 'vitest';

import { openWebpage } from '../js/async.mjs';
import { expect } from 'vitest';

test('should launch window', async (t) => {
  const abortController = new AbortController();
  const process = openWebpage(
    { url: 'https://example.com' },
    { cancelSignal: abortController.signal },
  );

  await new Promise((resolve) => setTimeout(resolve, 500));
  abortController.abort();

  await expect(process).rejects.toThrow('Command was canceled');
});
