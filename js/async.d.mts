import { type Options, ResultPromise } from 'execa';
import { type OpenWebpageOptions } from '../index.js';

export type { OpenWebpageOptions  } from '../index.js'

export function openWebpage(
  options: OpenWebpageOptions,
  execaOptions?: Options,
): ResultPromise;
