import { dirname, resolve } from 'path';
import type { OpenWebpageOptions } from '../index';
import { fileURLToPath } from 'url';

// const _require = global.require ?? createRequire(import.meta.url);
// const resolve = typeof require === 'function' ? require.resolve : import.meta.resolve;

const _dirname = typeof __dirname == 'string' ? __dirname : dirname(fileURLToPath(import.meta.url));

export async function openWebpage(options: OpenWebpageOptions = {}, execaOptions = {}) {
  const { execaNode } = await import('execa');

  return execaNode(
    execaOptions,
  )`${resolve(_dirname, './sync/open-webpage.mjs')} ${JSON.stringify(options)}`;
}
