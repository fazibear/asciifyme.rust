use wasm_bindgen::{prelude::*, Clamped};
use web_sys::{window, CanvasRenderingContext2d, HtmlCanvasElement, HtmlVideoElement};

const CHARS_LENGTH: usize = 16;
const CHARS: &str = "   .,:;i1tfLCG08@";

#[derive(Debug)]
pub struct Asciifyier {
    pub chars: Vec<char>,
    pub chars_length: usize,
}

impl Asciifyier {
    pub fn new() -> Self {
        Self {
            chars: CHARS.chars().collect(),
            chars_length: CHARS_LENGTH,
        }
    }

    pub fn asciify(self: &Self, data: &Clamped<Vec<u8>>) -> String {
        let mut output = String::new();

        for y in 1..40 {
            for x in 1..80 {
                let offset = (y * 80 + x) * 4;

                let red = data[offset];
                let green = data[offset + 1];
                let blue = data[offset + 2];
                //let alpha = data[offset+3]

                let brightness =
                    (0.3 * red as f32 + 0.59 * green as f32 + 0.11 * blue as f32) / 255.0;

                let char_index = CHARS_LENGTH - (brightness * CHARS_LENGTH as f32) as usize;

                output.push(self.chars[char_index]);
            }
            output.push('\n');
        }
        output
    }
}
