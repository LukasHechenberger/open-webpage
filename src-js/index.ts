import { dirname, resolve } from 'path';
import type { OpenWebpageOptions } from '../index';
import { fileURLToPath } from 'url';
import type { Options as ExecaOptions } from 'execa';

const _dirname = typeof __dirname == 'string' ? __dirname : dirname(fileURLToPath(import.meta.url));

export async function openWebpage(
  options: OpenWebpageOptions = {},
  /** Options for execa (optional) @see https://www.npmjs.com/package/execa */
  execaOptions: ExecaOptions = {},
) {
  const { execaNode } = await import('execa');

  return execaNode(
    execaOptions,
  )`${resolve(_dirname, './sync/open-webpage.mjs')} ${JSON.stringify(options)}`;
}
