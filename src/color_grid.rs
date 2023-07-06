use crate::block::Color;
use wasm_bindgen::prelude::*;
use cursive::{
    event::{Event, EventResult, Key},
    View,
    Printer,
};
#[wasm_bindgen]
pub struct ColorGrid {
    width: usize,
    height: usize,
    data: Vec<Color>,
}


#[wasm_bindgen]
extern {
    fn alert(s: &str);
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

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    fn draw_background(&self, printer: &Printer) {
        let width = self.width();
        let height = self.height();
        for j in 0..height {
            for i in 0..width {
                printer.with_color(self.data[width * j + i].to_cursive(), |printer| {
                    printer.print((2*i, j), "  ");
                });
            }
        }
    }
}

impl View for ColorGrid {
    fn draw(&self, printer: &Printer) {
        self.draw_background(printer);
    }

    fn required_size(&mut self, _constraint: cursive::Vec2) -> cursive::Vec2 {
        let width = self.width();
        let height = self.height();
        cursive::Vec2::new(10*width + 3, 10 * height + 10)
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        EventResult::Consumed(None)
    }
}