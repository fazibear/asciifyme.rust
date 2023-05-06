use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

mod asciifyier;
mod canvas;
mod web_cam;

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    let context = canvas::Canvas::new(80.0, 40.0);
    let web_cam = web_cam::WebCam::new();
    let asciifyier = asciifyier::Asciifyier::new();

    web_cam.setup();

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::new(move || {

        context.draw_image(&web_cam.video);
        let data = context.get_image_data();
        let output = asciifyier.asciify(&data);

        web_sys::console::log_1(&output.into());

        request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}
