(function() {
    var wasm;
    const __exports = {};


    let cachedTextEncoder = new TextEncoder('utf-8');

    let cachegetUint8Memory = null;
    function getUint8Memory() {
        if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
            cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
        }
        return cachegetUint8Memory;
    }

    let WASM_VECTOR_LEN = 0;

    function passStringToWasm(arg) {

        const buf = cachedTextEncoder.encode(arg);
        const ptr = wasm.__wbindgen_malloc(buf.length);
        getUint8Memory().set(buf, ptr);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }
    /**
    * @param {string} arg0
    * @returns {number}
    */
    __exports.new_lumberyard = function(arg0) {
        const ptr0 = passStringToWasm(arg0);
        const len0 = WASM_VECTOR_LEN;
        return wasm.new_lumberyard(ptr0, len0);
    };

    /**
    * @param {number} arg0
    * @returns {void}
    */
    __exports.delete_lumberyard = function(arg0) {
        return wasm.delete_lumberyard(arg0);
    };

    /**
    * @param {number} arg0
    * @returns {number}
    */
    __exports.lumberyard_width = function(arg0) {
        return wasm.lumberyard_width(arg0);
    };

    /**
    * @param {number} arg0
    * @returns {number}
    */
    __exports.lumberyard_height = function(arg0) {
        return wasm.lumberyard_height(arg0);
    };

    /**
    * @param {number} arg0
    * @returns {void}
    */
    __exports.tick_lumberyard = function(arg0) {
        return wasm.tick_lumberyard(arg0);
    };

    /**
    * @param {number} arg0
    * @param {number} arg1
    * @param {number} arg2
    * @returns {void}
    */
    __exports.render_lumberyard = function(arg0, arg1, arg2) {
        return wasm.render_lumberyard(arg0, arg1, arg2);
    };

    let cachedTextDecoder = new TextDecoder('utf-8');

    function getStringFromWasm(ptr, len) {
        return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
    }

    __exports.__wbg_error_cc95a3d302735ca3 = function(arg0, arg1) {
        let varg0 = getStringFromWasm(arg0, arg1);

        varg0 = varg0.slice();
        wasm.__wbindgen_free(arg0, arg1 * 1);

        console.error(varg0);
    };
    /**
    * @returns {void}
    */
    __exports.init = function() {
        return wasm.init();
    };

    /**
    * @param {number} arg0
    * @returns {number}
    */
    __exports.alloc_vec = function(arg0) {
        return wasm.alloc_vec(arg0);
    };

    /**
    * @param {number} arg0
    * @param {number} arg1
    * @returns {void}
    */
    __exports.dealloc_vec = function(arg0, arg1) {
        return wasm.dealloc_vec(arg0, arg1);
    };

    function init(path_or_module) {
        let instantiation;
        const imports = { './web18': __exports };
        if (path_or_module instanceof WebAssembly.Module) {
            instantiation = WebAssembly.instantiate(path_or_module, imports)
            .then(instance => {
            return { instance, module: path_or_module }
        });
    } else {
        const data = fetch(path_or_module);
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            instantiation = WebAssembly.instantiateStreaming(data, imports);
        } else {
            instantiation = data
            .then(response => response.arrayBuffer())
            .then(buffer => WebAssembly.instantiate(buffer, imports));
        }
    }
    return instantiation.then(({instance}) => {
        wasm = init.wasm = instance.exports;

    });
};
self.wasm_bindgen = Object.assign(init, __exports);
})();
