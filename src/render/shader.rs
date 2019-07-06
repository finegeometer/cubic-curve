use wasm_bindgen::prelude::*;

use std::rc::Rc;

type GL = web_sys::WebGl2RenderingContext;

pub struct Shader {
    gl: Rc<GL>,
    shader: web_sys::WebGlShader,
}

impl Drop for Shader {
    fn drop(&mut self) {
        self.gl.delete_shader(Some(&self.shader));
    }
}

impl Shader {
    pub fn new(gl: Rc<GL>, shader_type: u32, source: &str) -> Result<Self, JsValue> {
        let shader = gl
            .create_shader(shader_type)
            .ok_or("create_shader failed.")?;
        gl.shader_source(&shader, source);
        gl.compile_shader(&shader);

        if gl
            .get_shader_parameter(&shader, GL::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(Shader { gl, shader })
        } else {
            let log = gl
                .get_shader_info_log(&shader)
                .unwrap_or_else(|| "Unknown error creating shader".into());
            gl.delete_shader(Some(&shader));
            Err(log.into())
        }
    }
}

impl std::ops::Deref for Shader {
    type Target = web_sys::WebGlShader;
    fn deref(&self) -> &web_sys::WebGlShader {
        &self.shader
    }
}
