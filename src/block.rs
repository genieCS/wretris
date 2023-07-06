use wasm_bindgen::prelude::*;
use cursive::theme::{ BaseColor, ColorStyle, self };

#[wasm_bindgen]
#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    EMPTY = 0,
    HINT = 1,
    WARNING = 2,
    I = 3,
    O = 4,
    T = 5,
    S = 6,
    Z = 7,
    J = 8,
    L = 9,
}

impl Color {
    pub fn to_cursive(&self) -> ColorStyle {
        match self {
            Color::EMPTY => ColorStyle::new(theme::Color::Dark(BaseColor::Blue), theme::Color::Dark(BaseColor::Blue)),
            Color::HINT => ColorStyle::new(theme::Color::Dark(BaseColor::Blue), theme::Color::Dark(BaseColor::Blue)),
            Color::WARNING => ColorStyle::new(theme::Color::Dark(BaseColor::Blue), theme::Color::Dark(BaseColor::Blue)),
            Color::I => ColorStyle::new(theme::Color::Dark(BaseColor::Blue), theme::Color::Dark(BaseColor::Blue)),
            Color::O => ColorStyle::new(theme::Color::Dark(BaseColor::Yellow), theme::Color::Dark(BaseColor::Yellow)),
            Color::T => ColorStyle::new(theme::Color::Dark(BaseColor::Magenta), theme::Color::Dark(BaseColor::Magenta)),
            Color::S => ColorStyle::new(theme::Color::Dark(BaseColor::Green), theme::Color::Dark(BaseColor::Green)),
            Color::Z => ColorStyle::new(theme::Color::Dark(BaseColor::Red), theme::Color::Dark(BaseColor::Red)),
            Color::J => ColorStyle::new(theme::Color::Dark(BaseColor::Cyan), theme::Color::Dark(BaseColor::Cyan)),
            Color::L => ColorStyle::new(theme::Color::Dark(BaseColor::White), theme::Color::Dark(BaseColor::White)),
        }
    }
}