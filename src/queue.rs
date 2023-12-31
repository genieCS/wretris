use crate::block::{ Block, Shape };
use cursive:: {
    Printer,
    View,
    theme::{Color, ColorStyle},
};
use std::collections::VecDeque;

pub struct Queue {
    pub blocks: VecDeque<Block>,
    shapes: Vec<Shape>,
}

impl Default for Queue {
    fn default() -> Self {
        Self::new()
    }
}

impl Queue {
    pub fn new() -> Self {
        let mut blocks = VecDeque::new();
        for _ in 0..3 {
            blocks.push_back(Block::default());
        }
        Self {
            blocks,
            shapes: Shape::all(),
        }
    }

    pub fn renew(&mut self) {
        *self = Self::new();
    }

    pub fn pop_and_spawn_new_block(&mut self) -> Block {
        let block = self.blocks.pop_front().unwrap();
        if self.shapes.is_empty() {
            self.shapes = Shape::all();
        }
        self.blocks.push_back(Block::new(self.shapes.pop().unwrap()));
        block
    }

    fn draw_blocks(&self, printer: &Printer) {
        let mut y_padding = 2;
        for block in &self.blocks {
            for vector in &block.cells() {
                printer.with_color(block.color().to_cursive(), |printer| {
                    printer.print((5 + 2*vector.0, y_padding + vector.1), "  ");
                });
            }
            y_padding += 5;
        }
    }

    fn draw_container(&self, printer: &Printer) {
        let color_style = ColorStyle::new(Color::Rgb(255,255,255), Color::Rgb(183, 85, 224));
        for j in 0..15 {
                printer.with_color(color_style, |printer| {
                    printer.print((0, j), "|          |");
                });
        }
        printer.with_color(color_style, |printer| {
            printer.print((0, 15), "|__________|");
        });
    }
}

impl View for Queue {
    fn draw(&self, printer: &Printer) {
        self.draw_container(printer);
        self.draw_blocks(printer);
    }

    fn required_size(&mut self, _constraint: cursive::Vec2) -> cursive::Vec2 {
        cursive::Vec2::new(12, 16)
    }
}

