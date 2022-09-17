/*
 * Ugly work-around until found a way to "return an object with function" in neon.
 * It's likely not supported.
 */

const plugin = require("./index.node");

export default function localImport({ extension }) {
  if (!extension) {
    throw new Error("Extension is required");
  }

  return {
    name: plugin.name,
    transform(sourceCode) {
      return plugin.transform(extension, sourceCode);
    },
  };
}
