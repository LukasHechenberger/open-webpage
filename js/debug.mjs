import setupDebug from 'debug';
import { name as packageName } from '../package.json';

export const debug = setupDebug(packageName);
