(function() {
    var wasm;
    const __exports = {};
    /**
    * @param {number} arg0
    * @param {number} arg1
    * @param {number} arg2
    * @param {number} arg3
    * @param {number} arg4
    * @returns {boolean}
    */
    __exports.render_next_path = function(arg0, arg1, arg2, arg3, arg4) {
        return (wasm.render_next_path(arg0, arg1, arg2, arg3, arg4)) !== 0;
    };

    /**
    * @param {number} arg0
    * @param {number} arg1
    * @param {number} arg2
    * @param {number} arg3
    * @param {number} arg4
    * @param {number} arg5
    * @returns {boolean}
    */
    __exports.render_distance = function(arg0, arg1, arg2, arg3, arg4, arg5) {
        return (wasm.render_distance(arg0, arg1, arg2, arg3, arg4, arg5)) !== 0;
    };

    /**
    * @param {number} arg0
    * @param {number} arg1
    * @param {number} arg2
    * @param {boolean} arg3
    * @returns {void}
    */
    __exports.render_map = function(arg0, arg1, arg2, arg3) {
        return wasm.render_map(arg0, arg1, arg2, arg3);
    };

    /**
    * @param {number} arg0
    * @returns {number}
    */
    __exports.calculate_max_distance = function(arg0) {
        return wasm.calculate_max_distance(arg0);
    };

    /**
    * @param {number} arg0
    * @returns {number}
    */
    __exports.calculate_total_far_distances = function(arg0) {
        return wasm.calculate_total_far_distances(arg0);
    };

    /**
    * @param {number} arg0
    * @returns {number}
    */
    __exports.new_locations = function(arg0) {
        return wasm.new_locations(arg0);
    };

    /**
    * @param {number} arg0
    * @returns {void}
    */
    __exports.delete_locations = function(arg0) {
        return wasm.delete_locations(arg0);
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
    __exports.new_path = function(arg0) {
        const ptr0 = passStringToWasm(arg0);
        const len0 = WASM_VECTOR_LEN;
        return wasm.new_path(ptr0, len0);
    };

    /**
    * @param {number} arg0
    * @returns {void}
    */
    __exports.delete_path = function(arg0) {
        return wasm.delete_path(arg0);
    };

    /**
    * @param {number} arg0
    * @returns {number}
    */
    __exports.new_map = function(arg0) {
        return wasm.new_map(arg0);
    };

    /**
    * @param {number} arg0
    * @returns {void}
    */
    __exports.delete_map = function(arg0) {
        return wasm.delete_map(arg0);
    };

    /**
    * @param {number} arg0
    * @returns {number}
    */
    __exports.map_width = function(arg0) {
        return wasm.map_width(arg0);
    };

    /**
    * @param {number} arg0
    * @returns {number}
    */
    __exports.map_height = function(arg0) {
        return wasm.map_height(arg0);
    };

    /**
    * @param {number} arg0
    * @param {number} arg1
    * @returns {number}
    */
    __exports.new_path_iter = function(arg0, arg1) {
        return wasm.new_path_iter(arg0, arg1);
    };

    /**
    * @param {number} arg0
    * @returns {void}
    */
    __exports.delete_path_iter = function(arg0) {
        return wasm.delete_path_iter(arg0);
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
        const imports = { './web20': __exports };
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
