const { rollupPluginLocalImport } = require('../index')

const plugin = rollupPluginLocalImport();

console.log('Plugin', plugin);

const source = `
export { default } from './first-file';
export * from './second-file';
`;

const transformed = plugin.transform(source);

console.log('transformed', transformed);
