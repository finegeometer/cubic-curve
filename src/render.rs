mod program;
mod shader;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use self::program::Program;
use crate::utils::as_f32_array;
use std::rc::Rc;

type GL = web_sys::WebGl2RenderingContext;

const VERTEX_SHADER: &str = r#"#version 300 es

in vec2 coord;
out vec2 vcoord;

void main() {
    vcoord = coord * 2.0 - 1.0;
    gl_Position = vec4(vcoord, 0.0, 1.0);
}

"#;

const FRAGMENT_SHADER: &str = r#"#version 300 es

precision mediump float;

uniform float c[10];
uniform vec2 pt[9];
uniform float aspect_ratio;

in vec2 vcoord;
out vec4 color;

float line_half_width = 0.006;

void main() {
    float x = 3. * vcoord.x * aspect_ratio;
    float y = 3. * vcoord.y;

    float value =
        c[0]*x*x*x +
        c[1]*x*x*y +
        c[2]*x*y*y +
        c[3]*y*y*y +
        c[4]*x*x +
        c[5]*x*y +
        c[6]*y*y +
        c[7]*x +
        c[8]*y +
        c[9];

    vec2 derivative = vec2(
        c[0]*x*x*3.0 +
        c[1]*x*y*2.0 +
        c[2]*y*y +
        c[4]*x*2.0 +
        c[5]*y +
        c[7],
        c[1]*x*x +
        c[2]*x*y*2.0 +
        c[3]*y*y*3.0 +
        c[5]*x +
        c[6]*y*2.0 +
        c[8]);

    float brightness = exp(-(value*value) / (2.0 * dot(derivative,derivative) * line_half_width*line_half_width));

    color = mix(vec4(0.0,0.0,0.0,0.0), vec4(0.0,0.0,0.0,1.0), brightness);

    for (int i = 0; i < 9; i++) {
        if (distance(vec2(x,y), pt[i]) < 0.1) {
            color = mix(color, vec4(1.0, 0.0, 0.0, 1.0), 0.5);
        }
    }

}


"#;

pub type RenderFunction = dyn Fn([f32; 2], [[f32; 2]; 9], [f32; 10]);

pub fn make_fn(
    canvas: &web_sys::HtmlCanvasElement,
) -> Result<impl 'static + Fn([f32; 2], [[f32; 2]; 9], [f32; 10]), JsValue> {
    let gl = Rc::new(
        canvas
            .get_context("webgl2")?
            .ok_or("\"webgl2\" context identifier not supported.")?
            .dyn_into::<GL>()?,
    );

    let program = Program::new(Rc::clone(&gl), VERTEX_SHADER, FRAGMENT_SHADER)?;

    let coord_loc = program.attribute("coord")?;
    let coeff_loc = program.uniform("c")?;
    let point_loc = program.uniform("pt")?;
    let aspect_loc = program.uniform("aspect_ratio")?;

    let vao = gl
        .create_vertex_array()
        .ok_or("create_vertex_array failed")?;
    gl.bind_vertex_array(Some(&vao));

    let vertex_buffer = gl.create_buffer().ok_or("create_buffer failed")?;
    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
    gl.enable_vertex_attrib_array(coord_loc);
    gl.vertex_attrib_pointer_with_i32(coord_loc, 2, GL::FLOAT, false, 0, 0);
    gl.buffer_data_with_array_buffer_view(
        GL::ARRAY_BUFFER,
        &as_f32_array(&[0., 0., 0., 1., 1., 1., 1., 1., 1., 0., 0., 0.])?.into(),
        GL::STATIC_DRAW,
    );

    Ok(
        move |canvas_dimensions: [f32; 2], points: [[f32; 2]; 9], coeffs: [f32; 10]| {
            gl.bind_vertex_array(Some(&vao));

            gl.viewport(
                0,
                0,
                canvas_dimensions[0] as i32,
                canvas_dimensions[1] as i32,
            );
            gl.clear_color(0., 0., 0., 1.);
            gl.clear(GL::COLOR_BUFFER_BIT);

            gl.use_program(Some(&program));
            gl.bind_vertex_array(Some(&vao));

            gl.uniform1fv_with_f32_array(Some(&coeff_loc), &coeffs);
            gl.uniform2fv_with_f32_array(
                Some(&point_loc),
                &points.iter().flatten().cloned().collect::<Vec<_>>(),
            );
            gl.uniform1f(
                Some(&aspect_loc),
                canvas_dimensions[0] / canvas_dimensions[1],
            );

            gl.draw_arrays(GL::TRIANGLES, 0, 6);
        },
    )
}
