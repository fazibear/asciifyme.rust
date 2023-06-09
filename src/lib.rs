extern crate console_error_panic_hook;

mod asciifyier;
mod canvas;
mod utils;
mod web_cam;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlElement};

const CANVAS_WIDTH: u16 = 80;
const CANVAS_HEIGHT: u16 = 40;

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[wasm_bindgen(start)]
async fn run() -> Result<(), JsValue> {
    utils::set_panic_hook();
    console_log::init().expect("error initializing log");

    let context = canvas::Canvas::new();
    let web_cam = web_cam::WebCam::new();
    let pre = window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("pre")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();

    if let Ok(()) = web_cam.setup().await {
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        *g.borrow_mut() = Some(Closure::new(move || {
            request_animation_frame(f.borrow().as_ref().unwrap());

            context.draw_image(&web_cam.video);
            let data = context.get_image_data();
            let output = asciifyier::process(&data);

            pre.set_inner_text(&output);
        }));

        request_animation_frame(g.borrow().as_ref().unwrap());
    } else {
        pre.set_inner_text("Camera not found!");
    };

    Ok(())
}
