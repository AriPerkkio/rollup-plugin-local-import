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
