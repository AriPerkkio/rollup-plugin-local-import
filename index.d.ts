/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface TransformResult {
  code: string
}
export function localImport(extension: string): Plugin
export class Plugin {
  name: string
  extension: string
  transform(sourceCode: string): TransformResult
}
