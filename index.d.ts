interface Options {
  extension: string;
}

interface Plugin {
  name: "localImport";
  transform: (sourceCode: string) => { code: string };
}

export default function localImport(options: Options): Plugin;
