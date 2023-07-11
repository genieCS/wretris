use wasm_bindgen::prelude::*;

use crate::color_grid::{ ColorGrid, LR, };
use web_sys::console;

#[wasm_bindgen]
pub struct Board {
    grid: ColorGrid,
}
use cursive::{
    event::{Event, EventResult, Key, },
    Printer, Vec2, View,
};

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        Board {
            grid: ColorGrid::new(width, height),
        }
    }
    fn draw_background(&self, printer: &Printer) {
        // console::log_1(&"draw_background".into());
        let width = self.grid.width();
        let height = self.grid.height();
        for j in 0..height {
            for i in 0..width {
                printer.with_color(self.grid[width * j + i].to_cursive(), |printer| {
                    printer.print((2*i, j), "  ");
                });
            }
        }
    }

    fn draw_block(&self, printer: &Printer) {
        // console::log_1(&"draw_block".into());
        let width = self.grid.width();
        let height = self.grid.height();
        let pos = self.grid.block.pos;
        for (i, j) in self.grid.block.cells() {
            let x = pos.0 + i;
            let y = pos.1 + j;
            if x < 0 || x >= width as i32 || y < 0 || y >= height as i32 {
                continue;
            }
            printer.with_color(self.grid.block.to_cursive_color(), |printer| {
                printer.print((2*x as usize, y as usize), "  ");
            });
        }
    }

    pub fn renew(&self) {}

    pub fn on_down(&mut self, is_drop: bool, is_begin: bool) -> (bool, bool) {
        self.grid.on_down(is_drop, is_begin)
    }

    pub fn handle_event(&mut self, event: Event, hit_bottom: bool) -> bool {
        match event {
            Event::Key(Key::Left)  => self.grid.handle_lr(LR::Left, hit_bottom, false),
            Event::Key(Key::Right) => self.grid.handle_lr(LR::Right, hit_bottom, false),
            Event::Key(Key::Up) | Event::Char('e') => self.grid.rotate(hit_bottom, true),
            Event::Char('s') => self.grid.flip_turn(hit_bottom),
            Event::Char('w') => self.grid.rotate(hit_bottom, false),
            Event::Char('a') => self.grid.handle_lr(LR::Left, hit_bottom, true),
            Event::Char('d') => self.grid.handle_lr(LR::Right, hit_bottom, true),
            _ => false,
        }
    }
}

impl View for Board {
    fn draw(&self, printer: &Printer) {
        console::log_1(&"draw".into());
        self.draw_background(printer);
        self.draw_block(printer)
    }

    fn required_size(&mut self, _constraint: cursive::Vec2) -> cursive::Vec2 {
        Vec2::new(20,20)
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        self.handle_event(event, false);
        EventResult::Consumed(None)
    }
}
