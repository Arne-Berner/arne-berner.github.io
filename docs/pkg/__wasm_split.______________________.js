import { initSync } from "/pkg/arneberner.js";
function makeLoad(url, deps) {
  let alreadyLoaded = false;
  return async(callbackIndex, callbackData) => {
    if (alreadyLoaded) return;
    let mainExports = undefined;
      try {
        const loadAll = await Promise.all([fetch(url), ...deps.map(dep => dep())]);
        const response = loadAll[0];
        mainExports = initSync(undefined, undefined);
        const imports = {
          env: {
            memory: mainExports.memory,
          },
          __wasm_split: {
            __indirect_function_table: mainExports.__indirect_function_table,
            __stack_pointer: mainExports.__stack_pointer,
            __tls_base: mainExports.__tls_base,
            memory: mainExports.memory,
          },
        };
        const module = await WebAssembly.instantiateStreaming(response, imports);
        alreadyLoaded = true;
        if (callbackIndex === undefined) return;
        mainExports.__indirect_function_table.get(callbackIndex)(
          callbackData,
          true,
        );
      } catch (e) {
        if (callbackIndex === undefined) throw e;
        console.error("Failed to load " + url.href, e);
        if (mainExports === undefined) {
          mainExports = initSync(undefined, undefined);
        }
        mainExports.__indirect_function_table.get(callbackIndex)(
          callbackData,
          false,
        );
      }
  };
}
const __wasm_split_load_sub_counter_loader_8706884541029751135_window_height_loader_15579984902324837771_VtMkioU6kUTp3YbLlXs1Tw = makeLoad(new URL("./sub_counter_loader_8706884541029751135_window_height_loader_15579984902324837771.VtMkioU6kUTp3YbLlXs1Tw.wasm", import.meta.url), []);
export const __wasm_split_load_window_height_loader_15579984902324837771 = makeLoad(new URL("./window_height_loader_15579984902324837771.pZ9T-piG3CDr0aOICazUSA.wasm", import.meta.url), [__wasm_split_load_sub_counter_loader_8706884541029751135_window_height_loader_15579984902324837771_VtMkioU6kUTp3YbLlXs1Tw]);
export const __wasm_split_load_sub_counter_loader_8706884541029751135 = makeLoad(new URL("./sub_counter_loader_8706884541029751135.6mP4gJJ_w4whynxSPCa7Fw.wasm", import.meta.url), [__wasm_split_load_sub_counter_loader_8706884541029751135_window_height_loader_15579984902324837771_VtMkioU6kUTp3YbLlXs1Tw]);
