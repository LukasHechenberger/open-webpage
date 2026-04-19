import { MarkdownTemplate } from '@toolsync/template';
import { spawnSync } from 'child_process';
import { camelCase } from 'change-case';
import { OpenWebpageOptionsSchema } from '../src-js/__generated__/OpenWebpageOptions';

const codeblock = (lang, content) => `\`\`\`${lang}
${content}
\`\`\``;

export async function updateReadme() {
  const usage = spawnSync('node', ['./out/bin/index.mjs', '--help']).stdout.toString();

  const readme = await MarkdownTemplate.load('README.md');

  readme.update({
    section: 'cli-usage',
    content: codeblock('ansi', usage),
  });

  readme.update({
    section: 'sdk-usage',
    content: codeblock(
      'ts',
      `
import { openWebpage } from '@lhechenberger/open-webpage';

openWebpage(
  {
${Object.entries(OpenWebpageOptionsSchema.properties)
  .map(([name, schema]) => {
    const exampleValue = schema.type.includes('boolean') ? 'true' : "'some string'";
    return `    // ${schema.description}
    ${camelCase(name)}: ${exampleValue},`;
  })
  .join('\n')}
  },
  {
    // (optional) options for execa, see https://www.npmjs.com/package/execa
  },
);
`.trim(),
    ),
  });

  await readme.save();
}
