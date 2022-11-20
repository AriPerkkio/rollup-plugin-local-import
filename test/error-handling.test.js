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

test.todo("throws useful error message when callback throws", () => {
  const plugin = localImport(() => {
    throw new Error("Error from callback");
  });
  cleanups.push(plugin.buildEnd);

  expect(() =>
    plugin.transform('import fs from "./some-file";')
  ).toThrowErrorMatchingInlineSnapshot();
});
