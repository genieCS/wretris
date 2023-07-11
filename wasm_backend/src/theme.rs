use cursive_core::theme;

pub type Color = String;

#[derive(Debug)]
pub struct ColorPair {
    pub front: Color,
    pub back: Color,
}

pub fn cursive_to_color(c: theme::Color) -> Color {
    match c {
        theme::Color::Dark(theme::BaseColor::Black) => "#000000".to_string(),
        theme::Color::Dark(theme::BaseColor::Red) => "#800000".to_string(),
        theme::Color::Dark(theme::BaseColor::Green) => "#008000".to_string(),
        theme::Color::Dark(theme::BaseColor::Yellow) => "#808000".to_string(),
        theme::Color::Dark(theme::BaseColor::Blue) => "#000080".to_string(),
        theme::Color::Dark(theme::BaseColor::Magenta) => "#800080".to_string(),
        theme::Color::Dark(theme::BaseColor::Cyan) => "#008080".to_string(),
        theme::Color::Dark(theme::BaseColor::White) => "#c0c0c0".to_string(),
        theme::Color::Light(theme::BaseColor::Black) => "#808080".to_string(),
        theme::Color::Light(theme::BaseColor::Red) => "#ff0000".to_string(),
        theme::Color::Light(theme::BaseColor::Green) => "#00ff00".to_string(),
        theme::Color::Light(theme::BaseColor::Yellow) => "#ffff00".to_string(),
        theme::Color::Light(theme::BaseColor::Blue) => "#0000ff".to_string(),
        theme::Color::Light(theme::BaseColor::Magenta) => "#ff00ff".to_string(),
        theme::Color::Light(theme::BaseColor::Cyan) => "#00ffff".to_string(),
        theme::Color::Light(theme::BaseColor::White) => "#ffffff".to_string(),
        theme::Color::Rgb(r, g, b) => format!("#{:02x}{:02x}{:02x}", r, g, b).to_string(),
        theme::Color::RgbLowRes(r,g ,b ) => format!("#{:01x}{:01x}{:01x}", r, g, b).to_string(),
        theme::Color::TerminalDefault => "#000000".to_string(),
    }
}

pub fn cursive_to_color_pair(c: theme::ColorPair) -> ColorPair {
    ColorPair {
        front: cursive_to_color(c.front),
        back: cursive_to_color(c.back),
    }
}
