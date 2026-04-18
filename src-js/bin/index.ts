#!/usr/bin/env node

import { program } from 'commander';
import { version, name } from '../../package.json';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

program
  .name(`npx ${name}`)
  .version(version)
  .argument('<url>', 'The URL to open')
  .action(async (url, opts) => {
    const _dirname =
      typeof __dirname == 'string' ? __dirname : dirname(fileURLToPath(import.meta.url));
    const syncPath = resolve(_dirname, '../sync/open-webpage.mjs');

    const { execaNode } = await import('execa');

    await execaNode({})`${syncPath} ${JSON.stringify({ url, ...opts })}`;
  });

program.parseAsync();
