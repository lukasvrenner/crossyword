/* tslint:disable */
/* eslint-disable */
/**
* @returns {(PlacedWord)[] | undefined}
*/
export function create_puzzle(): (PlacedWord)[] | undefined;
/**
*/
export enum Orientation {
  Horizontal = 0,
  Vertical = 1,
}
/**
*/
export class PlacedWord {
  free(): void;
/**
*/
  readonly clue: string;
/**
*/
  orientation: Orientation;
/**
*/
  readonly word: string;
/**
*/
  xpos: number;
/**
*/
  ypos: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_placedword_free: (a: number) => void;
  readonly __wbg_get_placedword_orientation: (a: number) => number;
  readonly __wbg_set_placedword_orientation: (a: number, b: number) => void;
  readonly __wbg_get_placedword_xpos: (a: number) => number;
  readonly __wbg_set_placedword_xpos: (a: number, b: number) => void;
  readonly __wbg_get_placedword_ypos: (a: number) => number;
  readonly __wbg_set_placedword_ypos: (a: number, b: number) => void;
  readonly placedword_word: (a: number, b: number) => void;
  readonly placedword_clue: (a: number, b: number) => void;
  readonly create_puzzle: (a: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
