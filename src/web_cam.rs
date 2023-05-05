use wasm_bindgen::prelude::*;
use web_sys::{HtmlVideoElement, window};

#[derive(Debug)]
pub struct WebCam {
    pub video: HtmlVideoElement
}

impl WebCam {
    pub fn new() -> Self  {
        let document = window()
            .unwrap()
            .document()
            .unwrap();

        let video = document
            .create_element("video")
            .unwrap()
            .dyn_into::<web_sys::HtmlVideoElement>()
            .unwrap();

        Self {
            video: video
        }
    }

    pub fn setup(&self) {
        let video = self.video.clone();

        let can_play = Closure::wrap(Box::new(move |_: JsValue| {
            let _ = video.play().expect("should play");
        }) as Box<dyn FnMut(_)>);

        let video = self.video.clone();

        let on_stream = Closure::wrap(Box::new(move |stream: JsValue| {
            video.set_src_object(Some(&stream.into()));
            let _ = video.add_event_listener_with_callback("canplaythrough", can_play.as_ref().unchecked_ref());
        }) as Box<dyn FnMut(_)>);

        let mut constraints = web_sys::MediaStreamConstraints::new();
        constraints.video(&JsValue::from(true));

        let _ = web_sys::window()
            .unwrap()
            .navigator()
            .media_devices()
            .unwrap()
            .get_user_media_with_constraints(&constraints)
            .unwrap()
            .then(&on_stream);

    }
}
