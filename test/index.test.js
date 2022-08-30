const { localImport } = require('../index')

const plugin = localImport()

const source = `
export { default } from './first-file';
export * from './second-file';
`

const transformed = plugin.transform(source)

console.log('transformed', transformed)
