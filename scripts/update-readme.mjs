import { MarkdownTemplate } from '@toolsync/template';
import { spawnSync } from 'child_process';

export async function updateReadme() {
  const usage = spawnSync('node', ['./out/bin/index.mjs', '--help']).stdout.toString();

  const readme = await MarkdownTemplate.load('README.md');

  readme.update({
    section: 'cli-usage',
    content: `\`\`\`ansi
${usage}
\`\`\``,
  });

  await readme.save();
}
