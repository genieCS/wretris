use crate::numbers::padding;
use cursive::{
    theme::{ Color, ColorStyle,},
    View,
    Printer,
};
use js_sys::Date;


pub struct Timer {
    start: f64,
    is_paused: bool,
    pause_start: f64,
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

impl Timer {
    pub fn new() -> Self {
        let now = Date::now();
        Self {
            start: now,
            is_paused: false,
            pause_start: now,
        }
    }

    pub fn renew(&mut self) {
        let now = Date::now();
        self.start = now;
        self.pause_start = now;
        self.is_paused = false;
    }

    pub fn toggle_pause(&mut self) {
        if self.is_paused {
            self.start += Date::now() - self.pause_start;
        } else {
            self.pause_start = Date::now();
        }
        self.is_paused = !self.is_paused;
    }

    pub fn time2str(&self) -> String {
        let (mins, secs, mills) = self.elapsed();
        let mins = padding(mins as usize, 2);
        let secs = padding(secs as usize, 2);
        let mills = padding(mills as usize, 3);
        format!(" Time {}:{}:{} ", mins, secs, mills)
    }

    fn elapsed(&self) -> (u128, u128, u128) {
        let mills = if self.is_paused {
            (self.pause_start - self.start) as u128
        } else { (Date::now() - self.start) as u128 };
        let mins = mills / 60000;
        let secs = (mills % 60000) / 1000;
        let mills = mills % 1000;
        (mins, secs, mills)
    }
}

impl View for Timer {
    fn draw(&self, printer: &Printer) {
        let color_style = ColorStyle::new(Color::Rgb(50, 79, 54), Color::Rgb(255,255,255));
        printer.with_color(color_style, |printer| {
            printer.print((0, 0), &self.time2str());
        });
    }

    fn required_size(&mut self, _: cursive::Vec2) -> cursive::Vec2 {
        let line = self.time2str();
        cursive::Vec2::new(line.len(), 1)
    }
}