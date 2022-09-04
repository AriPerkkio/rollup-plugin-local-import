import { expect, test } from 'vitest'

import { localImport } from '../index'

const plugin = localImport()

test('ExportAll', () => {
  const transformed = plugin.transform('export * from "./second-file";')

  expect(transformed.code.trim()).toBe('export * from "./second-file.js";')
})
