use crate::core::charset::Charset;

pub fn brightness_to_char(px: u8, charset: Charset) -> char {
    let symbols = charset.symbols();
    let n = symbols.len();
    if n == 1 {
        return symbols[0];
    }
    let mut idx = (px as usize * n) / 256;
    if idx >= n {
        idx = n - 1;
    }
    symbols[idx]
}

pub fn frame_to_ascii(frame: &[u8], width: u32, height: u32, charset: Charset) -> String {
    let mut out = String::with_capacity(((width + 1) * height) as usize);
    for y in 0..height {
        for x in 0..width {
            let idx = (y * width + x) as usize;
            out.push(brightness_to_char(frame[idx], charset));
        }
        out.push('\n');
    }
    out
}