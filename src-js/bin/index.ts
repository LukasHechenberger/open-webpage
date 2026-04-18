#!/usr/bin/env node

import { program } from 'commander';
import { version, name } from '../../package.json';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { kebabCase } from 'change-case';
import { properties } from '../__generated__/options.schema.json';

program.name(`npx ${name}`).version(version);

for (const [_name, { description }] of Object.entries(properties)) {
  const name = kebabCase(_name);

  // Use url as an argument
  if (name === 'url') {
    program.argument(`[${name}]`, description);
  } else {
    program.option(`--${name}`, description);
  }
}

program
  .action(async function run(url, opts) {
    const _dirname =
      typeof __dirname == 'string' ? __dirname : dirname(fileURLToPath(import.meta.url));
    const syncPath = resolve(_dirname, '../sync/open-webpage.mjs');

    const { execaNode } = await import('execa');

    await execaNode({})`${syncPath} ${JSON.stringify({ url, ...opts })}`;
  })
  .parseAsync();
