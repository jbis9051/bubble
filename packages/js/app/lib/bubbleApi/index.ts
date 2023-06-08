import { NativeModules } from "react-native/types";

const { BubbleApi } = NativeModules;

interface IBubbleApi {
    addNumbers(a: number, b: number): number;
}

export default BubbleApi as IBubbleApi;
