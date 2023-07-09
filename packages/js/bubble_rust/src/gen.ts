/* WARNING: This file is auto-generated. Do not modify. */

import { NativeModules, Platform } from 'react-native';
import { Result } from './index';

const LINKING_ERROR =
    `The package 'react-native-bubble-rust' doesn't seem to be linked. Make sure: \n\n${Platform.select(
        { ios: "- You have run 'pod install'\n", default: '' }
    )}- You rebuilt the app after installing the package\n` +
    `- You are not using Expo Go\n`;

export const RustInterop = NativeModules.Bubble
    ? NativeModules.Bubble
    : new Proxy(
          {},
          {
              get() {
                  throw new Error(LINKING_ERROR);
              },
          }
      );

/* ---------------- STRUCT DEFINITIONS ------------------- */

export interface HelloResponse {
    message: string,
}

/* ---------------- FUNCTION DEFINITIONS ------------------- */

export function multiply(a: number , b: number ): Promise<Result<number, void>> {
    return RustInterop.call(JSON.stringify({
        method: 'multiply',
        args: {a, b},
    })).then((res: string) => JSON.parse(res));
}

    export function hello(name: string ): Promise<Result<HelloResponse, void>> {
    return RustInterop.call(JSON.stringify({
        method: 'hello',
        args: {name},
    })).then((res: string) => JSON.parse(res));
}

    