(function() {
    var wasm;
    const __exports = {};
    /**
    * @returns {void}
    */
    __exports.init = function() {
        return wasm.init();
    };

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
    __exports.new_ground = function(arg0) {
        const ptr0 = passStringToWasm(arg0);
        const len0 = WASM_VECTOR_LEN;
        return wasm.new_ground(ptr0, len0);
    };

    /**
    * @param {number} arg0
    * @returns {void}
    */
    __exports.delete_ground = function(arg0) {
        return wasm.delete_ground(arg0);
    };

    /**
    * @param {number} arg0
    * @returns {number}
    */
    __exports.ground_width = function(arg0) {
        return wasm.ground_width(arg0);
    };

    /**
    * @param {number} arg0
    * @returns {number}
    */
    __exports.ground_height = function(arg0) {
        return wasm.ground_height(arg0);
    };

    /**
    * @param {number} arg0
    * @returns {boolean}
    */
    __exports.tick_ground = function(arg0) {
        return (wasm.tick_ground(arg0)) !== 0;
    };

    /**
    * @param {number} arg0
    * @returns {number}
    */
    __exports.get_water_count = function(arg0) {
        return wasm.get_water_count(arg0);
    };

    /**
    * @param {number} arg0
    * @returns {number}
    */
    __exports.get_water_locked = function(arg0) {
        return wasm.get_water_locked(arg0);
    };

    /**
    * @param {number} arg0
    * @param {number} arg1
    * @param {number} arg2
    * @param {boolean} arg3
    * @returns {void}
    */
    __exports.render_ground = function(arg0, arg1, arg2, arg3) {
        return wasm.render_ground(arg0, arg1, arg2, arg3);
    };

    /**
    * @param {number} arg0
    * @returns {number}
    */
    __exports.alloc_img_data = function(arg0) {
        return wasm.alloc_img_data(arg0);
    };

    /**
    * @param {number} arg0
    * @param {number} arg1
    * @returns {void}
    */
    __exports.dealloc = function(arg0, arg1) {
        return wasm.dealloc(arg0, arg1);
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

    function init(path_or_module) {
        let instantiation;
        const imports = { './web17': __exports };
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
