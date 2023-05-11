use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlVideoElement, MediaStreamConstraints};

#[derive(Debug)]
pub struct WebCam {
    pub video: HtmlVideoElement,
}

impl WebCam {
    pub fn new() -> Self {
        let document = window().unwrap().document().unwrap();

        let video = document
            .create_element("video")
            .unwrap()
            .dyn_into::<HtmlVideoElement>()
            .unwrap();

        video.set_autoplay(true);

        Self { video }
    }

    pub async fn setup(&self) -> Result<(), JsValue> {
        let mut constraints = MediaStreamConstraints::new();
        constraints.video(&JsValue::from(true));

        let promise = window()
            .unwrap()
            .navigator()
            .media_devices()
            .unwrap()
            .get_user_media_with_constraints(&constraints)
            .unwrap();

        let stream = wasm_bindgen_futures::JsFuture::from(promise).await?;

        self.video.set_src_object(Some(&stream.into()));

        Ok(())
    }
}
