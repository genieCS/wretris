use crate::block::{ BlockWithPos, Color };
use wasm_bindgen::prelude::*;
use cursive::{
    event::{Event, EventResult, Key},
    View,
    Printer,
};
use web_sys::console;
#[wasm_bindgen]
pub struct ColorGrid {
    width: usize,
    height: usize,
    data: Vec<Color>,
    block: BlockWithPos,
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
            block: BlockWithPos::new(),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    fn draw_background(&self, printer: &Printer) {
        console::log_1(&JsValue::from_str(&"draw_background"));
        let width = self.width();
        let height = self.height();
        for i in 0..width {
            for j in 0..height {
                printer.with_color(self.data[width * j + i].to_cursive(), |printer| {
                    printer.print((2*i, j), "  ");
                });
            }
        }
    }

    fn draw_block(&self, printer: &Printer) {
        console::log_1(&JsValue::from_str(&"draw_block"));
        let width = self.width();
        let height = self.height();
        let pos = self.block.pos;
        for (i, j) in self.block.cells() {
            let x = pos.0 + i;
            let y = pos.1 + j;
            // console::log_1(&JsValue::from_str(&format!("width:{} height:{} x: {} y:{}", width, height, x, y)));
            if x < 0 || x >= width as i32 || y < 0 || y >= height as i32 {
                continue;
            }
            printer.with_color(self.block.to_cursive_color(), |printer| {
                // printer.print((2*x as usize, y as usize), &format!("{}{}", x, y));
                printer.print((2*x as usize, y as usize), "  ");
            });
        }
    }

    fn refresh(&mut self) {
        // self.data = vec![Color::S; 10 * 20];
    }
}

impl View for ColorGrid {
    fn draw(&self, printer: &Printer) {
        self.draw_background(printer);
        self.draw_block(printer);
    }

    fn required_size(&mut self, _constraint: cursive::Vec2) -> cursive::Vec2 {
        let width = self.width();
        let height = self.height();
        cursive::Vec2::new(10*width + 3, 10 * height + 10)
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        if event == Event::Refresh {
            self.refresh();
        }
        EventResult::Consumed(None)
    }
}