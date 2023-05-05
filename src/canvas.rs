use wasm_bindgen::prelude::*;
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement};

#[derive(Debug)]
pub struct Canvas {
    pub ctx: CanvasRenderingContext2d,
}

impl Canvas {
    pub fn new() -> Canvas {
        let document = window().unwrap().document().unwrap();

        let ctx = document
            .create_element("canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap()
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        Canvas { ctx }
    }
}
