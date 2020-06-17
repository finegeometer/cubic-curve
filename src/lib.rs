#![deny(unsafe_code)]

mod render;
mod utils;

use core::cell::RefCell;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

thread_local! {
    static MODEL: RefCell<Model> = RefCell::new(Model::init().unwrap_throw());

    static FRAME: js_sys::Function = {
        let closure: Closure<dyn FnMut(f64)> =
            Closure::wrap(Box::new(|timestamp| update(Msg::Frame(timestamp))));
        let animation_frame_closure = closure.as_ref().unchecked_ref::<js_sys::Function>().clone();
        closure.forget();
        animation_frame_closure
    }
}

fn update(msg: Msg) {
    MODEL.with(|m| m.borrow_mut().update(msg).unwrap_throw())
}

fn request_frame() {
    FRAME.with(|f| {
        web_sys::window()
            .unwrap_throw()
            .request_animation_frame(f)
            .unwrap_throw();
    })
}

#[wasm_bindgen]
pub fn run() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    request_frame()
}

struct Model {
    render: Box<render::RenderFunction>,
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
    Frame(f64),
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

        event_listener(&canvas, "click", |evt, model| {
            let evt = evt.dyn_into::<web_sys::MouseEvent>().unwrap_throw();
            Msg::Click(screen_coords(
                model.canvas_dimensions,
                [evt.offset_x(), evt.offset_y()],
            ))
        });
        event_listener(&canvas, "mousemove", |evt, model| {
            let evt = evt.dyn_into::<web_sys::MouseEvent>().unwrap_throw();
            Msg::MouseMove(screen_coords(
                model.canvas_dimensions,
                [evt.offset_x(), evt.offset_y()],
            ))
        });
        event_listener(&window, "resize", |_, _| Msg::Resize);

        Ok(Self {
            render,
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
            }
            Msg::MouseMove(pos) => {
                if let Some(p) = self.selected_point {
                    self.points[p] = pos;
                }
            }
            Msg::Resize => {
                self.canvas_dimensions = [
                    self.body.client_width() as f32 - 20.,
                    self.body.client_height() as f32 - 20.,
                ];
                self.canvas
                    .set_attribute("width", &format!("{}", self.canvas_dimensions[0]))?;
                self.canvas
                    .set_attribute("height", &format!("{}", self.canvas_dimensions[1]))?;
            }
            Msg::Frame(_) => {}
        }
        self.view();
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

    fn to_vec10([x, y]: [f32; 2]) -> na::VectorN<f64, na::U10> {
        let [x, y] = [f64::from(x), f64::from(y)];
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

    let matrix: na::MatrixMN<f64, na::U10, na::U9> =
        na::MatrixMN::from_columns(&points.iter().cloned().map(to_vec10).collect::<Vec<_>>());

    [
        matrix.remove_row(0).determinant() as f32,
        -matrix.remove_row(1).determinant() as f32,
        matrix.remove_row(2).determinant() as f32,
        -matrix.remove_row(3).determinant() as f32,
        matrix.remove_row(4).determinant() as f32,
        -matrix.remove_row(5).determinant() as f32,
        matrix.remove_row(6).determinant() as f32,
        -matrix.remove_row(7).determinant() as f32,
        matrix.remove_row(8).determinant() as f32,
        -matrix.remove_row(9).determinant() as f32,
    ]
}

fn event_listener(
    target: &web_sys::EventTarget,
    event: &str,
    msg: impl 'static + Fn(web_sys::Event, &Model) -> Msg,
) {
    let closure: Closure<dyn FnMut(web_sys::Event)> = Closure::wrap(Box::new(move |evt| {
        let msg = MODEL.with(|m| msg(evt, &m.borrow()));
        update(msg)
    }));
    target
        .add_event_listener_with_callback(event, closure.as_ref().unchecked_ref())
        .unwrap_throw();
    closure.forget()
}
