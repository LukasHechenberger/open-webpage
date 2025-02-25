import { defineConfig } from '@lhechenberger/tsup-dev';
import { getFilesToWatch } from '@lhechenberger/tsup-watch-monorepo';

export default defineConfig({
  entry: ['./src-js/index.ts', './src-js/sync/*.ts'],
  outDir: './out',
  dts: true,
  format: ['cjs', 'esm'],
  watch: await getFilesToWatch(),
  clean: true,
  external: ['../../index.js'],
});
