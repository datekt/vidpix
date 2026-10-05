use ab_glyph::{FontVec, PxScale};
use anyhow::{Context, Result};
use image::{Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;

const FONT_DATA: &[u8] = include_bytes!("../../assets/font/Datekt-Black.ttf");

pub const CELL_W: u32 = 14;
pub const CELL_H: u32 = 28;
pub const FONT_SIZE: f32 = 24.0;

pub struct Renderer {
    font: FontVec,
    scale: PxScale,
    width: u32,
    height: u32,
}

impl Renderer {
    pub fn new(width: u32, height: u32) -> Result<Self> {
        let font = FontVec::try_from_vec(FONT_DATA.to_vec())
            .context("failed to load embedded font")?;
        Ok(Self {
            font,
            scale: PxScale::from(FONT_SIZE),
            width,
            height,
        })
    }

    pub fn image_size(&self) -> (u32, u32) {
        (self.width * CELL_W, self.height * CELL_H)
    }

    pub fn render(&self, frame: &str) -> RgbaImage {
        let (w, h) = self.image_size();
        let mut img = RgbaImage::from_pixel(w, h, Rgba([0, 0, 0, 255]));

        for (y, line) in frame.lines().enumerate().take(self.height as usize) {
            let mut x = 0u32;
            for ch in line.chars().take(self.width as usize) {
                if ch != ' ' {
                    let buf = ch.to_string();
                    draw_text_mut(
                        &mut img,
                        Rgba([255, 255, 255, 255]),
                        x as i32,
                        (y as u32 * CELL_H) as i32,
                        self.scale,
                        &self.font,
                        &buf,
                    );
                }
                x += CELL_W;
            }
        }

        img
    }
}