use wasm_bindgen::{prelude::*, Clamped};
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement, HtmlVideoElement};

#[derive(Debug)]
pub struct Canvas {
	pub width: f64,
	pub height: f64,
    pub ctx: CanvasRenderingContext2d,
}

impl Canvas {
    pub fn new(width: f64, height: f64) -> Self {
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

        Self {
            width,
            height,
            ctx
        }
    }

    pub fn draw_image(self: &Self, video: &HtmlVideoElement) {
	    self
            .ctx
            .draw_image_with_html_video_element(video, self.height, self.width)
            .unwrap();
    }

    pub fn get_image_data(self: &Self) -> Clamped<Vec<u8>>{
        self
            .ctx
            .get_image_data(0.0, 0.0, self.width, self.height)
            .unwrap()
            .data()
    }
}
