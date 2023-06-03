import { NativeModules, Platform } from 'react-native';

const LINKING_ERROR =
    `The package 'react-native-bubble-rust' doesn't seem to be linked. Make sure: \n\n${Platform.select(
        { ios: "- You have run 'pod install'\n", default: '' }
    )}- You rebuilt the app after installing the package\n` +
    `- You are not using Expo Go\n`;

const RustInterop = NativeModules.Bubble
    ? NativeModules.Bubble
    : new Proxy(
          {},
          {
              get() {
                  throw new Error(LINKING_ERROR);
              },
          }
      );

export function rust_foo(): Promise<string> {
    return RustInterop.rust_foo();
}

export function init(dataDir: string): Promise<void> {
    return RustInterop.init(dataDir);
}

// all functions bellow use the `call` hack
export function multiply(a: number, b: number): Promise<string> {
    return RustInterop.call(JSON.stringify({
        method: 'multiply',
        args: { a, b },
    }));
}
