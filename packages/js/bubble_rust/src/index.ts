export * from "./gen";

export type Result<T, E>  = {
    success: true;
    value: T;
} | {
    success: false;
    value: E;
}
