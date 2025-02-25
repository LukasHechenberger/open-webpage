import { join } from 'path';
import { execaNode } from 'execa';

const dirname = new URL('.', import.meta.url).pathname;

export function openWebpage(options, execaOptions = {}) {
  return execaNode(
    execaOptions,
  )`${join(dirname, 'sync/open-webpage.mjs')} ${JSON.stringify(options)}`;
}
