use wasm_bindgen::Clamped;

const CHARS_LENGTH: usize = 16;
const fn get_char(index: usize) -> char {
    "   .,:;i1tfLCG08@".as_bytes()[index] as char
}

pub fn process(data: &Clamped<Vec<u8>>) -> String {
    let mut output = String::new();

    for y in 0..40 {
        for x in 0..80 {
            let offset = (y * 80 + x) * 4;

            let red = data[offset];
            let green = data[offset + 1];
            let blue = data[offset + 2];
            //let alpha = data[offset+3]

            let brightness = (0.3 * red as f32 + 0.59 * green as f32 + 0.11 * blue as f32) / 255.0;

            let char_index = CHARS_LENGTH - (brightness * CHARS_LENGTH as f32) as usize;

            output.push(get_char(char_index));
        }
        output.push('\n');
    }
    output
}
