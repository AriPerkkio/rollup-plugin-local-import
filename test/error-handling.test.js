import { expect, test } from "vitest";

import { localImport } from "../plugin";

test("throws useful error message when callback cleanup fails", () => {
  const plugin = localImport(() => {});

  // Run cleanup once. Next one should raise error
  plugin.buildEnd();

  expect(() => plugin.buildEnd()).toThrowErrorMatchingInlineSnapshot(
    '"Failed to cleanup callback. Unexpected Rollup lifecycle order."'
  );
});
