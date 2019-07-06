use wasm_bindgen::prelude::*;

use super::shader::Shader;
use std::rc::Rc;

type GL = web_sys::WebGl2RenderingContext;

pub struct Program {
    gl: Rc<GL>,
    program: web_sys::WebGlProgram,
}

impl Drop for Program {
    fn drop(&mut self) {
        self.gl.delete_program(Some(&self.program));
    }
}

impl Program {
    pub fn new(gl: Rc<GL>, vertex_shader: &str, fragment_shader: &str) -> Result<Self, JsValue> {
        let vertex_shader = Shader::new(Rc::clone(&gl), GL::VERTEX_SHADER, vertex_shader)?;
        let fragment_shader = Shader::new(Rc::clone(&gl), GL::FRAGMENT_SHADER, fragment_shader)?;
        let program = gl.create_program().ok_or("create_program failed")?;
        gl.attach_shader(&program, &vertex_shader);
        gl.attach_shader(&program, &fragment_shader);
        gl.link_program(&program);

        if gl
            .get_program_parameter(&program, GL::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(Program { gl, program })
        } else {
            let log = gl
                .get_program_info_log(&program)
                .unwrap_or_else(|| "Unknown error creating program".into());
            gl.delete_program(Some(&program));
            Err(log.into())
        }
    }

    pub fn uniform(&self, uniform: &str) -> Result<web_sys::WebGlUniformLocation, JsValue> {
        match self.gl.get_uniform_location(&self.program, uniform) {
            None => Err(format!("Uniform {} not found", uniform).into()),
            Some(x) => Ok(x),
        }
    }

    pub fn attribute(&self, attribute: &str) -> Result<u32, JsValue> {
        match self.gl.get_attrib_location(&self.program, attribute) {
            -1 => Err(format!("Attribute {} not found", attribute).into()),
            x => Ok(x as u32),
        }
    }
}

impl std::ops::Deref for Program {
    type Target = web_sys::WebGlProgram;
    fn deref(&self) -> &web_sys::WebGlProgram {
        &self.program
    }
}
