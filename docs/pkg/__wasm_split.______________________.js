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
const __wasm_split_load_sub_counter_loader_8706884541029751135_window_height_loader_15579984902324837771_5v7acYyu7XA7t00McpiVdQ = makeLoad(new URL("./sub_counter_loader_8706884541029751135_window_height_loader_15579984902324837771.5v7acYyu7XA7t00McpiVdQ.wasm", import.meta.url), []);
export const __wasm_split_load_window_height_loader_15579984902324837771 = makeLoad(new URL("./window_height_loader_15579984902324837771.bTy0Y-vw7oL7lpodKLAd-w.wasm", import.meta.url), [__wasm_split_load_sub_counter_loader_8706884541029751135_window_height_loader_15579984902324837771_5v7acYyu7XA7t00McpiVdQ]);
export const __wasm_split_load_sub_counter_loader_8706884541029751135 = makeLoad(new URL("./sub_counter_loader_8706884541029751135.nAHeqEXBTXwu1UR0y-mp7Q.wasm", import.meta.url), [__wasm_split_load_sub_counter_loader_8706884541029751135_window_height_loader_15579984902324837771_5v7acYyu7XA7t00McpiVdQ]);
