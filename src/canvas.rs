use wasm_bindgen::{prelude::*, Clamped};
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement, HtmlVideoElement};

use crate::{CANVAS_WIDTH, CANVAS_HEIGHT};

#[derive(Debug)]
pub struct Canvas {
    pub context: CanvasRenderingContext2d,
}

impl Canvas {
    pub fn new() -> Self {
        let document = window().unwrap().document().unwrap();

        let context = document
            .create_element("canvas")
            .unwrap()
            .dyn_into::<HtmlCanvasElement>()
            .unwrap()
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        Self {
            context
        }
    }

    pub fn draw_image(self: &Self, video: &HtmlVideoElement) {
	    self
            .context
            .draw_image_with_html_video_element(video, CANVAS_WIDTH as f64, CANVAS_HEIGHT as f64)
            .unwrap();
    }

    pub fn get_image_data(self: &Self) -> Clamped<Vec<u8>>{
        self
            .context
            .get_image_data(0.0, 0.0, CANVAS_HEIGHT as f64, CANVAS_WIDTH as f64)
            .unwrap()
            .data()
    }
}
