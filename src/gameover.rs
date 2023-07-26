use cursive::{
    event::{Callback, Event, EventResult, Key},
    View, Vec2,
    Printer,
    theme::{BaseColor, Color, ColorStyle},
};
use crate::tetris::Tetris;

pub struct GameOver {}

impl Default for GameOver {
    fn default() -> Self {
        Self::new()
    }
}

impl GameOver {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl View for GameOver {
    fn draw(&self, printer: &Printer) {
        for y in 0..printer.size.y {
            for x in 0..printer.size.x {
                printer.with_color(ColorStyle::new(Color::Dark(BaseColor::Blue), Color::Dark(BaseColor::Blue)), |printer| {
                    printer.print((x, y), " ");
                });
            }
        }
        printer.with_color(ColorStyle::new(Color::Dark(BaseColor::White), Color::Dark(BaseColor::Blue)), |printer| {
            printer.print((10, 2), "game over");
        });
    }

    fn required_size(&mut self, _constraint: cursive::Vec2) -> cursive::Vec2 {
        Vec2::new(20, 5)
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        if event != Event::Key(Key::Enter) {
            EventResult::Ignored
        } else {
            EventResult::Consumed(Some(Callback::from_fn(move |s| {
                s.pop_layer();
                s.call_on_name("tetris", |t: &mut Tetris| t.on_event(Event::Char('n')));
            })))
        }
    }
}
