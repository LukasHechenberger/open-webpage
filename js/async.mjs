import { execaNode } from 'execa';
import { createRequire } from 'module';

const require = createRequire(import.meta.url);

export function openWebpage(options, execaOptions = {}) {
  return execaNode(
    execaOptions,
  )`${require.resolve('./sync/open-webpage.mjs')} ${JSON.stringify(options)}`;
}
