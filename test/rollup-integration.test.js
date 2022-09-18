import { rmSync, writeFileSync } from "fs";
import { afterAll, beforeAll, expect, test } from "vitest";
import { rollup } from "rollup";

import { localImport } from "../plugin";

const input = "input.js";
const output = { file: "output.js" };
const source = `
// ExportAllDeclaration
export * from "./local-file";
export * from "../file-from-parent-directory";
export * from 'some-dependency';

// ExportNamedDeclaration
export { a } from "./local-file";
export { b } from "../file-from-parent-directory";
export { c } from 'some-dependency';
`.trim();

beforeAll(() => {
  writeFileSync(input, source, "utf-8");
});

afterAll(() => {
  rmSync(input);
  rmSync(output.file);
});

test("rollup", async () => {
  const build = await rollup({
    input,
    external: () => true,
    plugins: [localImport(".js")],
  });

  const bundle = await build.write({ output });

  expect(bundle.output).toHaveLength(1);
  expect(bundle.output[0].code.trim()).toMatchInlineSnapshot(`
    "export * from './local-file.js';
    export { a } from './local-file.js';
    export * from '../file-from-parent-directory.js';
    export { b } from '../file-from-parent-directory.js';
    export * from 'some-dependency';
    export { c } from 'some-dependency';"
  `);
});

test("plugin has name", () => {
  expect(localImport(".js")).toHaveProperty("name", "local-import");
});
