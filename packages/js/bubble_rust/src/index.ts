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

export function init(dataDir: string): Promise<void> {
    return RustInterop.init(dataDir);
}
