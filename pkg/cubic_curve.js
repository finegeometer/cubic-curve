
let wasm;

/**
* @returns {void}
*/
export function run() {
    return wasm.run();
}

const heap = new Array(32);

heap.fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

let cachedTextDecoder = new TextDecoder('utf-8');

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

function getStringFromWasm(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

let WASM_VECTOR_LEN = 0;

let cachedTextEncoder = new TextEncoder('utf-8');

let passStringToWasm;
if (typeof cachedTextEncoder.encodeInto === 'function') {
    passStringToWasm = function(arg) {


        let size = arg.length;
        let ptr = wasm.__wbindgen_malloc(size);
        let offset = 0;
        {
            const mem = getUint8Memory();
            for (; offset < arg.length; offset++) {
                const code = arg.charCodeAt(offset);
                if (code > 0x7F) break;
                mem[ptr + offset] = code;
            }
        }

        if (offset !== arg.length) {
            arg = arg.slice(offset);
            ptr = wasm.__wbindgen_realloc(ptr, size, size = offset + arg.length * 3);
            const view = getUint8Memory().subarray(ptr + offset, ptr + size);
            const ret = cachedTextEncoder.encodeInto(arg, view);

            offset += ret.written;
        }
        WASM_VECTOR_LEN = offset;
        return ptr;
    };
} else {
    passStringToWasm = function(arg) {


        let size = arg.length;
        let ptr = wasm.__wbindgen_malloc(size);
        let offset = 0;
        {
            const mem = getUint8Memory();
            for (; offset < arg.length; offset++) {
                const code = arg.charCodeAt(offset);
                if (code > 0x7F) break;
                mem[ptr + offset] = code;
            }
        }

        if (offset !== arg.length) {
            const buf = cachedTextEncoder.encode(arg.slice(offset));
            ptr = wasm.__wbindgen_realloc(ptr, size, size = offset + buf.length);
            getUint8Memory().set(buf, ptr + offset);
            offset += buf.length;
        }
        WASM_VECTOR_LEN = offset;
        return ptr;
    };
}

let cachegetUint32Memory = null;
function getUint32Memory() {
    if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== wasm.memory.buffer) {
        cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
    }
    return cachegetUint32Memory;
}

function handleError(e) {
    wasm.__wbindgen_exn_store(addHeapObject(e));
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachegetFloat32Memory = null;
function getFloat32Memory() {
    if (cachegetFloat32Memory === null || cachegetFloat32Memory.buffer !== wasm.memory.buffer) {
        cachegetFloat32Memory = new Float32Array(wasm.memory.buffer);
    }
    return cachegetFloat32Memory;
}

function getArrayF32FromWasm(ptr, len) {
    return getFloat32Memory().subarray(ptr / 4, ptr / 4 + len);
}

function init(module) {
    if (typeof module === 'undefined') {
        module = import.meta.url.replace(/\.js$/, '_bg.wasm');
    }
    let result;
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        let varg0 = getStringFromWasm(arg0, arg1);
        return addHeapObject(varg0);
    };
    imports.wbg.__wbindgen_cb_forget = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbindgen_object_clone_ref = function(arg0) {
        return addHeapObject(getObject(arg0));
    };
    imports.wbg.__wbg_new_59cb74e423758ede = function() {
        return addHeapObject(new Error());
    };
    imports.wbg.__wbg_stack_558ba5917b466edd = function(ret, arg0) {

        const retptr = passStringToWasm(getObject(arg0).stack);
        const retlen = WASM_VECTOR_LEN;
        const mem = getUint32Memory();
        mem[ret / 4] = retptr;
        mem[ret / 4 + 1] = retlen;

    };
    imports.wbg.__wbg_error_4bb6c2a97407129a = function(arg0, arg1) {
        let varg0 = getStringFromWasm(arg0, arg1);

        varg0 = varg0.slice();
        wasm.__wbindgen_free(arg0, arg1 * 1);

        console.error(varg0);
    };
    imports.wbg.__widl_instanceof_Window = function(arg0) {
        return getObject(arg0) instanceof Window;
    };
    imports.wbg.__widl_f_set_property_CSSStyleDeclaration = function(arg0, arg1, arg2, arg3, arg4) {
        let varg1 = getStringFromWasm(arg1, arg2);
        let varg3 = getStringFromWasm(arg3, arg4);
        try {
            getObject(arg0).setProperty(varg1, varg3);
        } catch (e) {
            handleError(e);
        }
    };
    imports.wbg.__widl_f_create_element_Document = function(arg0, arg1, arg2) {
        let varg1 = getStringFromWasm(arg1, arg2);
        try {
            return addHeapObject(getObject(arg0).createElement(varg1));
        } catch (e) {
            handleError(e);
        }
    };
    imports.wbg.__widl_f_body_Document = function(arg0) {

        const val = getObject(arg0).body;
        return isLikeNone(val) ? 0 : addHeapObject(val);

    };
    imports.wbg.__widl_f_set_attribute_Element = function(arg0, arg1, arg2, arg3, arg4) {
        let varg1 = getStringFromWasm(arg1, arg2);
        let varg3 = getStringFromWasm(arg3, arg4);
        try {
            getObject(arg0).setAttribute(varg1, varg3);
        } catch (e) {
            handleError(e);
        }
    };
    imports.wbg.__widl_f_client_width_Element = function(arg0) {
        return getObject(arg0).clientWidth;
    };
    imports.wbg.__widl_f_client_height_Element = function(arg0) {
        return getObject(arg0).clientHeight;
    };
    imports.wbg.__widl_f_add_event_listener_with_callback_EventTarget = function(arg0, arg1, arg2, arg3) {
        let varg1 = getStringFromWasm(arg1, arg2);
        try {
            getObject(arg0).addEventListener(varg1, getObject(arg3));
        } catch (e) {
            handleError(e);
        }
    };
    imports.wbg.__widl_instanceof_HTMLCanvasElement = function(arg0) {
        return getObject(arg0) instanceof HTMLCanvasElement;
    };
    imports.wbg.__widl_f_get_context_HTMLCanvasElement = function(arg0, arg1, arg2) {
        let varg1 = getStringFromWasm(arg1, arg2);
        try {

            const val = getObject(arg0).getContext(varg1);
            return isLikeNone(val) ? 0 : addHeapObject(val);

        } catch (e) {
            handleError(e);
        }
    };
    imports.wbg.__widl_instanceof_HTMLElement = function(arg0) {
        return getObject(arg0) instanceof HTMLElement;
    };
    imports.wbg.__widl_f_set_inner_text_HTMLElement = function(arg0, arg1, arg2) {
        let varg1 = getStringFromWasm(arg1, arg2);
        getObject(arg0).innerText = varg1;
    };
    imports.wbg.__widl_f_style_HTMLElement = function(arg0) {
        return addHeapObject(getObject(arg0).style);
    };
    imports.wbg.__widl_instanceof_MouseEvent = function(arg0) {
        return getObject(arg0) instanceof MouseEvent;
    };
    imports.wbg.__widl_f_offset_x_MouseEvent = function(arg0) {
        return getObject(arg0).offsetX;
    };
    imports.wbg.__widl_f_offset_y_MouseEvent = function(arg0) {
        return getObject(arg0).offsetY;
    };
    imports.wbg.__widl_f_append_child_Node = function(arg0, arg1) {
        try {
            return addHeapObject(getObject(arg0).appendChild(getObject(arg1)));
        } catch (e) {
            handleError(e);
        }
    };
    imports.wbg.__widl_instanceof_WebGL2RenderingContext = function(arg0) {
        return getObject(arg0) instanceof WebGL2RenderingContext;
    };
    imports.wbg.__widl_f_bind_vertex_array_WebGL2RenderingContext = function(arg0, arg1) {
        getObject(arg0).bindVertexArray(getObject(arg1));
    };
    imports.wbg.__widl_f_buffer_data_with_array_buffer_view_WebGL2RenderingContext = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).bufferData(arg1 >>> 0, getObject(arg2), arg3 >>> 0);
    };
    imports.wbg.__widl_f_create_vertex_array_WebGL2RenderingContext = function(arg0) {

        const val = getObject(arg0).createVertexArray();
        return isLikeNone(val) ? 0 : addHeapObject(val);

    };
    imports.wbg.__widl_f_uniform1fv_with_f32_array_WebGL2RenderingContext = function(arg0, arg1, arg2, arg3) {
        let varg2 = getArrayF32FromWasm(arg2, arg3);
        getObject(arg0).uniform1fv(getObject(arg1), varg2);
    };
    imports.wbg.__widl_f_uniform2fv_with_f32_array_WebGL2RenderingContext = function(arg0, arg1, arg2, arg3) {
        let varg2 = getArrayF32FromWasm(arg2, arg3);
        getObject(arg0).uniform2fv(getObject(arg1), varg2);
    };
    imports.wbg.__widl_f_attach_shader_WebGL2RenderingContext = function(arg0, arg1, arg2) {
        getObject(arg0).attachShader(getObject(arg1), getObject(arg2));
    };
    imports.wbg.__widl_f_bind_buffer_WebGL2RenderingContext = function(arg0, arg1, arg2) {
        getObject(arg0).bindBuffer(arg1 >>> 0, getObject(arg2));
    };
    imports.wbg.__widl_f_clear_WebGL2RenderingContext = function(arg0, arg1) {
        getObject(arg0).clear(arg1 >>> 0);
    };
    imports.wbg.__widl_f_clear_color_WebGL2RenderingContext = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).clearColor(arg1, arg2, arg3, arg4);
    };
    imports.wbg.__widl_f_compile_shader_WebGL2RenderingContext = function(arg0, arg1) {
        getObject(arg0).compileShader(getObject(arg1));
    };
    imports.wbg.__widl_f_create_buffer_WebGL2RenderingContext = function(arg0) {

        const val = getObject(arg0).createBuffer();
        return isLikeNone(val) ? 0 : addHeapObject(val);

    };
    imports.wbg.__widl_f_create_program_WebGL2RenderingContext = function(arg0) {

        const val = getObject(arg0).createProgram();
        return isLikeNone(val) ? 0 : addHeapObject(val);

    };
    imports.wbg.__widl_f_create_shader_WebGL2RenderingContext = function(arg0, arg1) {

        const val = getObject(arg0).createShader(arg1 >>> 0);
        return isLikeNone(val) ? 0 : addHeapObject(val);

    };
    imports.wbg.__widl_f_delete_program_WebGL2RenderingContext = function(arg0, arg1) {
        getObject(arg0).deleteProgram(getObject(arg1));
    };
    imports.wbg.__widl_f_delete_shader_WebGL2RenderingContext = function(arg0, arg1) {
        getObject(arg0).deleteShader(getObject(arg1));
    };
    imports.wbg.__widl_f_draw_arrays_WebGL2RenderingContext = function(arg0, arg1, arg2, arg3) {
        getObject(arg0).drawArrays(arg1 >>> 0, arg2, arg3);
    };
    imports.wbg.__widl_f_enable_vertex_attrib_array_WebGL2RenderingContext = function(arg0, arg1) {
        getObject(arg0).enableVertexAttribArray(arg1 >>> 0);
    };
    imports.wbg.__widl_f_get_attrib_location_WebGL2RenderingContext = function(arg0, arg1, arg2, arg3) {
        let varg2 = getStringFromWasm(arg2, arg3);
        return getObject(arg0).getAttribLocation(getObject(arg1), varg2);
    };
    imports.wbg.__widl_f_get_program_info_log_WebGL2RenderingContext = function(ret, arg0, arg1) {
        const val = getObject(arg0).getProgramInfoLog(getObject(arg1));
        const retptr = isLikeNone(val) ? [0, 0] : passStringToWasm(val);
        const retlen = WASM_VECTOR_LEN;
        const mem = getUint32Memory();
        mem[ret / 4] = retptr;
        mem[ret / 4 + 1] = retlen;

    };
    imports.wbg.__widl_f_get_program_parameter_WebGL2RenderingContext = function(arg0, arg1, arg2) {
        return addHeapObject(getObject(arg0).getProgramParameter(getObject(arg1), arg2 >>> 0));
    };
    imports.wbg.__widl_f_get_shader_info_log_WebGL2RenderingContext = function(ret, arg0, arg1) {
        const val = getObject(arg0).getShaderInfoLog(getObject(arg1));
        const retptr = isLikeNone(val) ? [0, 0] : passStringToWasm(val);
        const retlen = WASM_VECTOR_LEN;
        const mem = getUint32Memory();
        mem[ret / 4] = retptr;
        mem[ret / 4 + 1] = retlen;

    };
    imports.wbg.__widl_f_get_shader_parameter_WebGL2RenderingContext = function(arg0, arg1, arg2) {
        return addHeapObject(getObject(arg0).getShaderParameter(getObject(arg1), arg2 >>> 0));
    };
    imports.wbg.__widl_f_get_uniform_location_WebGL2RenderingContext = function(arg0, arg1, arg2, arg3) {
        let varg2 = getStringFromWasm(arg2, arg3);

        const val = getObject(arg0).getUniformLocation(getObject(arg1), varg2);
        return isLikeNone(val) ? 0 : addHeapObject(val);

    };
    imports.wbg.__widl_f_link_program_WebGL2RenderingContext = function(arg0, arg1) {
        getObject(arg0).linkProgram(getObject(arg1));
    };
    imports.wbg.__widl_f_shader_source_WebGL2RenderingContext = function(arg0, arg1, arg2, arg3) {
        let varg2 = getStringFromWasm(arg2, arg3);
        getObject(arg0).shaderSource(getObject(arg1), varg2);
    };
    imports.wbg.__widl_f_uniform1f_WebGL2RenderingContext = function(arg0, arg1, arg2) {
        getObject(arg0).uniform1f(getObject(arg1), arg2);
    };
    imports.wbg.__widl_f_use_program_WebGL2RenderingContext = function(arg0, arg1) {
        getObject(arg0).useProgram(getObject(arg1));
    };
    imports.wbg.__widl_f_vertex_attrib_pointer_with_i32_WebGL2RenderingContext = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        getObject(arg0).vertexAttribPointer(arg1 >>> 0, arg2, arg3 >>> 0, arg4 !== 0, arg5, arg6);
    };
    imports.wbg.__widl_f_viewport_WebGL2RenderingContext = function(arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).viewport(arg1, arg2, arg3, arg4);
    };
    imports.wbg.__widl_f_request_animation_frame_Window = function(arg0, arg1) {
        try {
            return getObject(arg0).requestAnimationFrame(getObject(arg1));
        } catch (e) {
            handleError(e);
        }
    };
    imports.wbg.__widl_f_document_Window = function(arg0) {

        const val = getObject(arg0).document;
        return isLikeNone(val) ? 0 : addHeapObject(val);

    };
    imports.wbg.__wbg_call_836fa928f74337e5 = function(arg0, arg1) {
        try {
            return addHeapObject(getObject(arg0).call(getObject(arg1)));
        } catch (e) {
            handleError(e);
        }
    };
    imports.wbg.__wbg_newnoargs_8d1797b163dbc9fb = function(arg0, arg1) {
        let varg0 = getStringFromWasm(arg0, arg1);
        return addHeapObject(new Function(varg0));
    };
    imports.wbg.__wbg_buffer_e04d67bf3bf41917 = function(arg0) {
        return addHeapObject(getObject(arg0).buffer);
    };
    imports.wbg.__wbg_new_28b28665890d8497 = function(arg0) {
        return addHeapObject(new Float32Array(getObject(arg0)));
    };
    imports.wbg.__wbg_subarray_604f76ccca176864 = function(arg0, arg1, arg2) {
        return addHeapObject(getObject(arg0).subarray(arg1 >>> 0, arg2 >>> 0));
    };
    imports.wbg.__wbg_instanceof_Memory_7cf0cf614421960b = function(arg0) {
        return getObject(arg0) instanceof WebAssembly.Memory;
    };
    imports.wbg.__wbindgen_boolean_get = function(arg0) {
        const v = getObject(arg0);
        return typeof(v) === 'boolean' ? (v ? 1 : 0) : 2;
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        let varg0 = getStringFromWasm(arg0, arg1);
        throw new Error(varg0);
    };
    imports.wbg.__wbindgen_memory = function() {
        return addHeapObject(wasm.memory);
    };
    imports.wbg.__wbindgen_closure_wrapper105 = function(arg0, arg1, arg2) {

        const f = wasm.__wbg_function_table.get(27);
        const d = wasm.__wbg_function_table.get(28);
        const b = arg1;
        const cb = function(arg0) {
            this.cnt++;
            let a = this.a;
            this.a = 0;
            try {
                return f(a, b, addHeapObject(arg0));

            } finally {
                if (--this.cnt === 0) d(a, b);
                else this.a = a;

            }

        };
        cb.a = arg0;
        cb.cnt = 1;
        let real = cb.bind(cb);
        real.original = cb;

        return addHeapObject(real);
    };
    imports.wbg.__wbindgen_closure_wrapper107 = function(arg0, arg1, arg2) {

        const f = wasm.__wbg_function_table.get(31);
        const d = wasm.__wbg_function_table.get(28);
        const b = arg1;
        const cb = function(arg0) {
            this.cnt++;
            let a = this.a;
            this.a = 0;
            try {
                return f(a, b, arg0);

            } finally {
                if (--this.cnt === 0) d(a, b);
                else this.a = a;

            }

        };
        cb.a = arg0;
        cb.cnt = 1;
        let real = cb.bind(cb);
        real.original = cb;

        return addHeapObject(real);
    };

    if (module instanceof URL || typeof module === 'string' || module instanceof Request) {

        const response = fetch(module);
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            result = WebAssembly.instantiateStreaming(response, imports)
            .catch(e => {
                console.warn("`WebAssembly.instantiateStreaming` failed. Assuming this is because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);
                return response
                .then(r => r.arrayBuffer())
                .then(bytes => WebAssembly.instantiate(bytes, imports));
            });
        } else {
            result = response
            .then(r => r.arrayBuffer())
            .then(bytes => WebAssembly.instantiate(bytes, imports));
        }
    } else {

        result = WebAssembly.instantiate(module, imports)
        .then(result => {
            if (result instanceof WebAssembly.Instance) {
                return { instance: result, module };
            } else {
                return result;
            }
        });
    }
    return result.then(({instance, module}) => {
        wasm = instance.exports;
        init.__wbindgen_wasm_module = module;

        return wasm;
    });
}

export default init;

