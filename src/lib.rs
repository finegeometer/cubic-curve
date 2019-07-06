#![deny(unsafe_code)]

mod render;
mod utils;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    let state: Rc<RefCell<Model>> = Rc::new(RefCell::new(Model::init()?));

    let model = state.borrow_mut();

    {
        let state = state.clone();
        let closure: Closure<dyn FnMut(web_sys::MouseEvent)> =
            Closure::wrap(Box::new(move |evt| {
                let dims = state.borrow().canvas_dimensions;
                state
                    .borrow_mut()
                    .update(Msg::Click(screen_coords(
                        dims,
                        [evt.offset_x(), evt.offset_y()],
                    )))
                    .unwrap_or_else(|err| wasm_bindgen::throw_val(err));
            }));
        model
            .canvas
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let state = state.clone();
        let closure: Closure<dyn FnMut(web_sys::MouseEvent)> =
            Closure::wrap(Box::new(move |evt| {
                let dims = state.borrow().canvas_dimensions;
                state
                    .borrow_mut()
                    .update(Msg::MouseMove(screen_coords(
                        dims,
                        [evt.offset_x(), evt.offset_y()],
                    )))
                    .unwrap_or_else(|err| wasm_bindgen::throw_val(err));
            }));
        model
            .canvas
            .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let state = state.clone();
        let closure: Closure<dyn FnMut(web_sys::MouseEvent)> =
            Closure::wrap(Box::new(move |_evt| {
                state
                    .borrow_mut()
                    .update(Msg::Resize)
                    .unwrap_or_else(|err| wasm_bindgen::throw_val(err));
            }));
        model
            .window
            .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    model.view();

    Ok(())
}

struct Model {
    render: Box<render::RenderFunction>,
    window: web_sys::Window,
    body: web_sys::HtmlElement,
    canvas: web_sys::HtmlCanvasElement,
    canvas_dimensions: [f32; 2],
    text_box: web_sys::HtmlElement,
    points: [[f32; 2]; 9],
    selected_point: Option<usize>,
}

enum Msg {
    Click([f32; 2]),
    MouseMove([f32; 2]),
    Resize,
}

impl Model {
    fn init() -> Result<Self, JsValue> {
        let window = web_sys::window().ok_or("no global `window` exists")?;
        let document = window
            .document()
            .ok_or("should have a document on window")?;
        let body = document.body().ok_or("document should have a body")?;

        body.style().set_property("position", "relative")?;

        let canvas_dimensions = [
            body.client_width() as f32 - 20.,
            body.client_height() as f32 - 20.,
        ];

        let canvas = document
            .create_element("canvas")?
            .dyn_into::<web_sys::HtmlCanvasElement>()?;

        canvas.style().set_property("position", "absolute")?;
        canvas.style().set_property("top", "0")?;
        canvas.style().set_property("left", "0")?;

        canvas.set_attribute("width", &format!("{}", canvas_dimensions[0]))?;
        canvas.set_attribute("height", &format!("{}", canvas_dimensions[1]))?;

        body.append_child(&canvas)?;

        let text_box = document
            .create_element("p")?
            .dyn_into::<web_sys::HtmlElement>()?;
        body.append_child(&text_box)?;

        let render = Box::new(render::make_fn(&canvas)?);

        Ok(Self {
            render,
            window,
            body,
            canvas,
            canvas_dimensions,
            text_box,
            points: [
                [1.96, -2.744],
                [1.44, -1.728],
                [1.00, -1.000],
                [0.49, -0.343],
                [0.00, -0.000],
                [0.49, 0.343],
                [1.00, 1.000],
                [1.44, 1.728],
                [1.96, 2.744],
            ],
            selected_point: None,
        })
    }
    fn view(&self) {
        let mut coeffs = cubic(self.points);

        let mut max: f32 = std::f32::MIN;
        for &x in coeffs.iter() {
            if x.abs() >= max {
                max = x.abs();
            }
        }

        for c in coeffs.iter_mut() {
            *c /= max;
        }

        (self.render)(self.canvas_dimensions, self.points, coeffs);
        self.text_box.set_inner_text(&format!(
            "{:.2} x^3 + {:.2} x^2 y + {:.2} x y^2 + {:.2} y^3 + {:.2} x^2 + {:.2} x y + {:.2} y^2 + {:.2} x + {:.2} y + {:.2} = 0",
            coeffs[0], coeffs[1], coeffs[2], coeffs[3], coeffs[4], coeffs[5], coeffs[6], coeffs[7], coeffs[8], coeffs[9],
        ));
    }
    fn update(&mut self, msg: Msg) -> Result<(), JsValue> {
        match msg {
            Msg::Click([x, y]) => {
                if self.selected_point.is_some() {
                    self.selected_point = None;
                } else {
                    for p in 0..9 {
                        if (self.points[p][0] - x) * (self.points[p][0] - x)
                            + (self.points[p][1] - y) * (self.points[p][1] - y)
                            < 0.01
                        {
                            self.selected_point = Some(p);
                            break;
                        }
                    }
                }
                self.view()
            }
            Msg::MouseMove(pos) => {
                if let Some(p) = self.selected_point {
                    self.points[p] = pos;
                }
                self.view()
            }
            Msg::Resize => {
                self.canvas_dimensions = [
                    self.body.client_width() as f32 - 20.,
                    self.body.client_height() as f32 - 20.,
                ];
                web_sys::console::log_1(&format!("{:?}", self.canvas_dimensions).into());
                self.canvas
                    .set_attribute("width", &format!("{}", self.canvas_dimensions[0]))?;
                self.canvas
                    .set_attribute("height", &format!("{}", self.canvas_dimensions[1]))?;
                self.view()
            }
        }
        Ok(())
    }
}

fn screen_coords([width, height]: [f32; 2], [x, y]: [i32; 2]) -> [f32; 2] {
    [
        ((x as f32 / width) * 2. - 1.) * 3. * (width / height),
        ((y as f32 / height) * 2. - 1.) * -3.,
    ]
}

fn cubic(points: [[f32; 2]; 9]) -> [f32; 10] {
    use nalgebra as na;

    fn to_vec10([x, y]: [f32; 2]) -> na::VectorN<f32, na::U10> {
        na::VectorN::from_row_slice_generic(
            na::U10,
            na::U1,
            &[
                x * x * x,
                x * x * y,
                x * y * y,
                y * y * y,
                x * x,
                x * y,
                y * y,
                x,
                y,
                1.,
            ],
        )
    }

    let matrix: na::MatrixMN<f32, na::U10, na::U9> =
        na::MatrixMN::from_columns(&points.iter().cloned().map(to_vec10).collect::<Vec<_>>());

    [
        matrix.remove_row(0).determinant(),
        -matrix.remove_row(1).determinant(),
        matrix.remove_row(2).determinant(),
        -matrix.remove_row(3).determinant(),
        matrix.remove_row(4).determinant(),
        -matrix.remove_row(5).determinant(),
        matrix.remove_row(6).determinant(),
        -matrix.remove_row(7).determinant(),
        matrix.remove_row(8).determinant(),
        -matrix.remove_row(9).determinant(),
    ]
}
