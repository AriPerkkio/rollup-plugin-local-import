# rollup-plugin-local-import

[![Version][version-badge]][version-url]

[Installation](#installation) | [Configuration](#configuration) | [Examples](#examples)

---

> [Rollup](https://rollupjs.org/) plugin for manipulating local `import`/`export` statements

[version-badge]: https://img.shields.io/npm/v/rollup-plugin-local-import
[version-url]: https://www.npmjs.com/package/rollup-plugin-local-import

## Installation

`rollup-plugin-local-import` should be included in development dependencies.

```
yarn add --dev rollup-plugin-local-import
```

### Supported Operating Systems

This plugin is built as native addon and supports following operating systems.

|                | node14 | node16 | node18 |
| -------------- | ------ | ------ | ------ |
| Windows x64    | ✓      | ✓      | ✓      |
| Windows x32    | ✓      | ✓      | ✓      |
| macOS x64      | ✓      | ✓      | ✓      |
| macOS arm64    | ✓      | ✓      | ✓      |
| Linux x64 gnu  | ✓      | ✓      | ✓      |
| Linux x64 musl | ✓      | ✓      | ✓      |

Support can be extended to cover more systems if needed.

## Configuration

Add plugin in your rollup configuration:

```js
import { defineConfig } from "rollup";
import { localImport } from "rollup-plugin-local-import";

export default defineConfig({
  // ...
  plugins: [localImport(".js")],
});
```

### Options

```ts
function localImport(extension: string): RollupPlugin;
```

- `extension`, required

The extension added to each identified local import.

```diff
// When extension is 'js'
- export { default } from './Header';
+ export { default } from './Header.js';
```

## Examples

With `localImport(".js")`:

Input:

```js
export * from "./local-file";
export * from "../file-from-parent-directory";
export * from "some-dependency";

export { a } from "./local-file";
export { b } from "../file-from-parent-directory";
export { c } from "some-dependency";
```

Output:

```js
export * from "./local-file.js";
export * from "../file-from-parent-directory.js";
export * from "some-dependency"; // Not changed

export { a } from "./local-file.js";
export { b } from "../file-from-parent-directory.js";
export { c } from "some-dependency"; // Not changed
```
