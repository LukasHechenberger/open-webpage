import { type Options, ResultPromise } from 'execa';
import { openWebpage as actual } from '../index.js';

export function openWebpage(
  options: Parameters<typeof actual>[0],
  execaOptions?: Options,
): ResultPromise;
