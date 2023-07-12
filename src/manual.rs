use cursive::{
    event::{Event, EventResult},
    theme::{ ColorStyle, Color, },
    Printer, View, Vec2,
};

pub struct Manual {
}

impl Default for Manual {
    fn default() -> Self {
        Self::new()
    }
}

impl Manual {
    pub fn new() -> Manual {
        Manual {}
    }
}

impl View for Manual {
    fn draw(&self, printer: &Printer) {
        let color_style = ColorStyle::new(Color::Rgb(50, 79, 54), Color::Rgb(255,255,255));
        printer.with_color(color_style, |printer| {
            printer.print((0, 0), &format!(" {:26} ", "Manual"));
            printer.print((0, 1), &format!(" {:26} ", "↑,e: rotate clockwise"));
            printer.print((0, 2), &format!(" {:26} ", "w: rotate counterclockwise"));
            printer.print((0, 3), &format!(" {:26} ", "s: flip turn"));
            printer.print((0, 4), &format!(" {:26} ", "↓: speed up"));
            printer.print((0, 5), &format!(" {:26} ", "←: left"));
            printer.print((0, 6), &format!(" {:26} ", "a: left most"));
            printer.print((0, 7), &format!(" {:26} ", "→: right"));
            printer.print((0, 8), &format!(" {:26} ", "d: right most"));
            printer.print((0, 9), &format!(" {:26} ", "space: hard drop"));
            printer.print((0, 10), &format!(" {:26} ", "m: stop and resume"));
            printer.print((0, 11), &format!(" {:26} ", "n: new game"));
        });
    }

    fn required_size(&mut self, _constraints: Vec2) -> Vec2 {
        Vec2::new(30, 16)
    }

    fn on_event(&mut self, _: Event) -> EventResult {
        EventResult::Ignored
    }
}
