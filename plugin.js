const { localImport } = require("./index");

// Work-around for "TypeError: Illegal invocation"
// Rollup calls plugins using ".apply", https://github.com/rollup/rollup/blob/8db7fd8e5c4079760bb034a28f476e210e8bf78d/src/utils/PluginDriver.ts#L324
function callContextBindHack(...args) {
  const plugin = localImport(...args);

  return {
    name: plugin.name,
    transform: plugin.transform.bind(plugin),
  };
}

module.exports.localImport = callContextBindHack;
