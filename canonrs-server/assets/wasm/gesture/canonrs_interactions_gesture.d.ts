/* tslint:disable */
/* eslint-disable */

export function init_gesture(el: Element): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly init_gesture: (a: any) => void;
    readonly wasm_bindgen__closure__destroy__h99a23b2d23cef8bd: (a: number, b: number) => void;
    readonly wasm_bindgen__closure__destroy__h2f9671a3481ed567: (a: number, b: number) => void;
    readonly wasm_bindgen__closure__destroy__h2c31b217641b77da: (a: number, b: number) => void;
    readonly wasm_bindgen__closure__destroy__hfcfdc426315b311f: (a: number, b: number) => void;
    readonly wasm_bindgen__closure__destroy__h0f1ec615cbc582ba: (a: number, b: number) => void;
    readonly wasm_bindgen__closure__destroy__h31fee2c60038af36: (a: number, b: number) => void;
    readonly wasm_bindgen__closure__destroy__he636d43b28fb6abf: (a: number, b: number) => void;
    readonly wasm_bindgen__closure__destroy__h03f29193b9f26a8e: (a: number, b: number) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h33870864403f15f6: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h2268dcb35aa6a45b: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__hd4d9f1a2eee58ac4: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__he80af123afa21e08: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__hb6a2b6a8e912a8ed: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__hdf5617c0626bbcc6: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h4155352c08969eab: (a: number, b: number, c: any) => void;
    readonly wasm_bindgen__convert__closures_____invoke__h4a607910ea31f4e9: (a: number, b: number) => void;
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
