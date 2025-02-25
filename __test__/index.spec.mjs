import { test, expect } from 'vitest';
import { openWebpage } from '../out/index.mjs';

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
