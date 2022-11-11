import { rmSync, writeFileSync } from "fs";
import { afterAll, expect, test } from "vitest";
import { rollup } from "rollup";

import { localImport } from "../plugin";

const input = "input.js";
const output = { file: "output.js" };

afterAll(() => {
  rmSync(input);
  rmSync(output.file);
});

test("ExportAllDeclaration", async () => {
  const output = await run(`
    export * from "./local-file";
    export * from "../file-from-parent-directory";
    export * from 'some-dependency';
  `);

  expect(output).toMatchInlineSnapshot(`
    "export * from './local-file.js';
    export * from '../file-from-parent-directory.js';
    export * from 'some-dependency';
    "
  `);
});

test("ExportNamedDeclaration", async () => {
  const output = await run(`
    export { a } from "./local-file";
    export { b } from "../file-from-parent-directory";
    export { c } from 'some-dependency';
  `);

  expect(output).toMatchInlineSnapshot(`
    "export { a } from './local-file.js';
    export { b } from '../file-from-parent-directory.js';
    export { c } from 'some-dependency';
    "
  `);
});

test("ImportDeclaration, default", async () => {
  const output = await run(`
    import a from "./local-file";
    import b from "../file-from-parent-directory";
    import c from 'some-dependency';
    console.log(a,b,c);
  `);

  expect(output).toMatchInlineSnapshot(`
    "import a from './local-file.js';
    import b from '../file-from-parent-directory.js';
    import c from 'some-dependency';

    console.log(a, b, c);
    "
  `);
});

test("ImportDeclaration, named", async () => {
  const output = await run(`
    import { a } from "./local-file";
    import { b } from "../file-from-parent-directory";
    import { c } from 'some-dependency';
    console.log(a,b,c);
  `);

  expect(output).toMatchInlineSnapshot(`
    "import { a } from './local-file.js';
    import { b } from '../file-from-parent-directory.js';
    import { c } from 'some-dependency';

    console.log(a, b, c);
    "
  `);
});

test("ImportDeclaration, side-effect", async () => {
  const output = await run(`
    import "./local-file";
    import "../file-from-parent-directory";
    import 'some-dependency';
  `);

  expect(output).toMatchInlineSnapshot(`
    "import './local-file.js';
    import '../file-from-parent-directory.js';
    import 'some-dependency';
    "
  `);
});

test("re-export named import", async () => {
  const output = await run(`
    import { sideEffects } from "./some-file";

    sideEffects();

    export { sideEffects };
  `);

  expect(output).toMatchInlineSnapshot(`
    "import { sideEffects } from './some-file.js';
    export { sideEffects } from './some-file.js';

    sideEffects();
    "
  `);
});

test("plugin has name", () => {
  const plugin = localImport(() => {});

  expect(plugin).toHaveProperty("name", "local-import");

  // Manual cleanup
  plugin.buildEnd();
});

async function run(source) {
  writeFileSync(input, source.trim(), "utf-8");

  const build = await rollup({
    input,
    external: () => true,
    plugins: [
      localImport((path) => {
        return `${path}.js`;
      }),
    ],
  });

  const bundle = await build.write(output);

  return bundle.output[0].code;
}
