import { openWebpage } from '../../index.js';

const [optionsJson] = process.argv.slice(2);

openWebpage(JSON.parse(optionsJson));
