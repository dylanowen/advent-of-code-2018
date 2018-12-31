// The `--no-modules`-generated JS from `wasm-bindgen` attempts to use
// `WebAssembly.instantiateStreaming` to instantiate the wasm module,
// but this doesn't work with `file://` urls. This example is frequently
// viewed by simply opening `index.html` in a browser (with a `file://`
// url), so it would fail if we were to call this function!
//
// Work around this for now by deleting the function to ensure that the
// `no_modules.js` script doesn't have access to it. You won't need this
// hack when deploying over HTTP.
delete WebAssembly.instantiateStreaming;


const LibReady = new Promise((resolve) => window.addEventListener('load', resolve)).then(async () => {
    await wasm_bindgen('./pkg/web_bg.wasm');
    wasm_bindgen.init();

    console.log("Lib Loaded");
    return wasm_bindgen;
});