use wasm_bindgen::prelude::*;

mod canvas;
mod web_cam;

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    let context = canvas::Canvas::new();
    let web_cam = web_cam::WebCam::new();
    web_cam.setup();

    context.ctx;

    web_sys::console::log_1(&"Hello using webs".into());
    Ok(())
}
