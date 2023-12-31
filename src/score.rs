use crate::numbers::padding;

use cursive::{
    theme::{ Color, ColorStyle,},
    View,
    Printer,
};

pub struct Score {
    score: usize,
    perfect: usize,
}

impl Default for Score {
    fn default() -> Self {
        Self {
            score: 0,
            perfect: 40,
        }
    }
}

impl Score {
    pub fn new() -> Score{
        Self::default()
    }

    pub fn add(&mut self, s: usize) {
        self.score += s
    }

    pub fn is_gameover(&self) -> bool {
        self.score >= self.perfect
    }

    pub fn renew(&mut self) {
        self.score = 0;
    }

    fn num2str(&self) -> String {
        format!(" Lines: {} / {} ", padding(self.score, 2), self.perfect)
    }
}

impl View for Score {
    fn draw(&self, printer: &Printer) {
        let color_style = ColorStyle::new(Color::Rgb(50, 79, 54), Color::Rgb(255,255,255));
        printer.with_color(color_style, |printer| {
            printer.print((0, 0), &self.num2str());
        });
    }

    fn required_size(&mut self, _constraint: cursive::Vec2) -> cursive::Vec2 {
        let line = self.num2str();
        cursive::Vec2::new(line.len() + 3, 1)
    }
}
