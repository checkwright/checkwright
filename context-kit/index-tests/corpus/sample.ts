export function alpha() {}

export async function beta(): Promise<void> {}

export class Gamma<T> {}

export interface Delta {
    x: number;
}

export type Epsilon = number;

export enum Zeta {
    A,
    B,
}

export const enum Eta {
    X,
}

export const theta = 1;

export let iota = 2;

export var kappa = 3;

export default function main() {}

export { alpha as renamedAlpha } from "./other";
export * from "./more";

const notExported = 4;
function alsoPrivate() {}
