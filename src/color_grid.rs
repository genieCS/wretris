use std::fmt::Display;
use crate::block::Color;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ColorGrid {
    width: usize,
    height: usize,
    data: Vec<Color>,
}

#[wasm_bindgen]
impl ColorGrid {
    pub fn new() -> ColorGrid {
        ColorGrid {
            width: 10,
            height: 20,
            data: vec![Color::EMPTY; 10 * 20],
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn data(&self) -> *const Color {
        self.data.as_ptr()
    }
}

impl Display for ColorGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        for y in 0..self.height {
            for x in 0..self.width {
                s.push_str(&format!("{:?} ", self.data[y * self.width + x]));
            }
            s.push('\n');
        }

        write!(f, "{}", s)
    }
}