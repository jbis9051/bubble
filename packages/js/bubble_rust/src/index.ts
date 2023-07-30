import { InitOptions, RustInterop } from './gen';

export * from './gen';

export type Result<T, E> =
    | {
          success: true;
          value: T;
      }
    | {
          success: false;
          value: E;
      };

export type Uuid = string;
export type Base64 = string;

export function init(
    options: InitOptions
): Promise<Result<FrontendInstance, string>> {
    return RustInterop.init(JSON.stringify(options)).then((res: string) =>
        JSON.parse(res)
    );
}

export function getAppDir(): Promise<string> {
    return RustInterop.getAppDir();
}

declare const tag: unique symbol;
export type FrontendInstance = number & { readonly [tag]: unique symbol };
