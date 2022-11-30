import { afterEach, expect, test } from "vitest";

import { localImport } from "../plugin";

const cleanups = [];

afterEach(() => {
  cleanups.splice(0).forEach((cleanup) => cleanup());
});

test("throws useful error message when callback cleanup fails", () => {
  const plugin = localImport(() => {});

  // Run cleanup once. Next one should raise error
  plugin.buildEnd();

  expect(() => plugin.buildEnd()).toThrowErrorMatchingInlineSnapshot(
    '"Failed to cleanup callback. Unexpected Rollup lifecycle order."'
  );
});

test("throws useful error message when callback is missing", () => {
  expect(() => localImport()).toThrowErrorMatchingInlineSnapshot(
    '"Failed to reference callback. Did you pass function to `localImport(callback)`?"'
  );
});

test("throws useful error message when callback is not function", () => {
  expect(() => localImport(123)).toThrowErrorMatchingInlineSnapshot(
    '"Failed to reference callback. Did you pass function to `localImport(callback)`?"'
  );
});

test("throws useful error message when callback throws", () => {
  const plugin = localImport(() => {
    throw new Error("Throwing some error from callback");
  });
  cleanups.push(plugin.buildEnd);

  expect(() =>
    plugin.transform('import A from "./some-file";', "file.js")
  ).toThrowErrorMatchingInlineSnapshot('"Run into 1 error(s): [\\"Callback threw error \\\\\\"Error: Throwing some error from callback\\\\\\" when called with \\\\\\"./some-file\\\\\\"\\"]."');
});

test("includes all errors thrown by callback in error message", () => {
  const plugin = localImport(() => {
    throw new Error("Throwing some error from callback");
  });
  cleanups.push(plugin.buildEnd);

  expect(() =>
    plugin.transform(
      `
    export * from "./local-file-1";
    export { a } from "./local-file-2";
    import b from "./local-file-3";
    import { c } from "./local-file-4";
    import "./local-file-5";
    `,
      "file.js"
    )
  ).toThrowErrorMatchingInlineSnapshot('"Run into 5 error(s): [\\"Callback threw error \\\\\\"Error: Throwing some error from callback\\\\\\" when called with \\\\\\"./local-file-1\\\\\\",Callback threw error \\\\\\"Error: Throwing some error from callback\\\\\\" when called with \\\\\\"./local-file-2\\\\\\",Callback threw error \\\\\\"Error: Throwing some error from callback\\\\\\" when called with \\\\\\"./local-file-3\\\\\\",Callback threw error \\\\\\"Error: Throwing some error from callback\\\\\\" when called with \\\\\\"./local-file-4\\\\\\",Callback threw error \\\\\\"Error: Throwing some error from callback\\\\\\" when called with \\\\\\"./local-file-5\\\\\\"\\"]."');
});
