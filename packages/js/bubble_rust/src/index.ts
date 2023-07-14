import { RustInterop } from './gen';

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

export function init(dataDir: string): Promise<Result<FrontendInstance, string>> {
    return RustInterop.init(dataDir).then((res: string) => JSON.parse(res));
}

declare const tag: unique symbol;
export type FrontendInstance = number & { readonly [tag]: unique symbol };
