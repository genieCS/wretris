use wasm_bindgen::prelude::*;
use cursive::{
    theme::{ BaseColor, ColorStyle, self },
};



pub type Pos = (i32, i32);

pub struct BlockWithPos {
    pub block: Block,
    pub pos: Pos,
}

impl BlockWithPos {
    pub fn new() -> Self {
        BlockWithPos { 
            block: Block
            {
                shape: Shape::J,
                rotation: Rotation::R0,
            },
            pos: (2, 2)
         }
    }

    pub fn to_cursive_color(&self) -> ColorStyle {
        self.block.shape.to_cursive()
    }

    pub fn cells(&self) -> Vec<Pos> {
        self.block.cells()
    }
}

pub struct Block 
{
    shape: Shape,
    rotation: Rotation,
}

impl Block {
    fn cells(&self) -> Vec<Pos> {
        match self.rotation {
            Rotation::R0 => self.shape.cells(),
            Rotation::R90 => self.shape.cells().into_iter().map(|(x,y)| (-y,x)).collect(),
            Rotation::R180 => self.shape.cells().into_iter().map(|(x,y)| (-x,-y)).collect(),
            Rotation::R270 => self.shape.cells().into_iter().map(|(x,y)| (y,-x)).collect(),
        }
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shape {
    I = 0,
    O = 1,
    T = 2,
    S = 3,
    Z = 4,
    J = 5,
    L = 6,
}

impl Shape {
    pub fn to_cursive(&self) -> ColorStyle {
        self.to_color().to_cursive()
    }

    fn to_color(&self) -> Color {
        match self {
            Shape::I => Color::I,
            Shape::O => Color::O,
            Shape::T => Color::T,
            Shape::S => Color::S,
            Shape::Z => Color::Z,
            Shape::J => Color::J,
            Shape::L => Color::L,
        }
    }

    fn cells(&self) -> Vec<Pos> {
        match self {
            Shape::I => vec![(0,0),(-2,0),(-1,0),(1,0)],
            Shape::O => vec![(0,0),(-1,-1),(0,-1),(-1,0)],
            Shape::T => vec![(0,0),(-1,0),(0,-1),(1,0)],
            Shape::S => vec![(0,0),(-1,0),(0,-1),(1,-1)],
            Shape::Z => vec![(0,0),(1,0),(0,-1),(-1,-1)],
            Shape::J => vec![(0,0),(-1,-1),(-1,0),(1,0)],
            Shape::L => vec![(0,0),(-1,0),(1,0),(1,-1)],
        }
    }
}

#[wasm_bindgen]
#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    I = 0,
    O = 1,
    T = 2,
    S = 3,
    Z = 4,
    J = 5,
    L = 6,
    EMPTY = 7,
    HINT = 8,
    WARNING = 9,
}

impl Color {
    pub fn to_cursive(&self) -> ColorStyle {
        match self {
            Color::I => ColorStyle::new(theme::Color::Dark(BaseColor::Blue), theme::Color::Dark(BaseColor::Blue)),
            Color::O => ColorStyle::new(theme::Color::Dark(BaseColor::Yellow), theme::Color::Dark(BaseColor::Yellow)),
            Color::T => ColorStyle::new(theme::Color::Dark(BaseColor::Magenta), theme::Color::Dark(BaseColor::Magenta)),
            Color::S => ColorStyle::new(theme::Color::Dark(BaseColor::Green), theme::Color::Dark(BaseColor::Green)),
            Color::Z => ColorStyle::new(theme::Color::Dark(BaseColor::Red), theme::Color::Dark(BaseColor::Red)),
            Color::J => ColorStyle::new(theme::Color::Dark(BaseColor::Cyan), theme::Color::Dark(BaseColor::Cyan)),
            Color::L => ColorStyle::new(theme::Color::Dark(BaseColor::White), theme::Color::Dark(BaseColor::White)),
            _ => ColorStyle::new(theme::Color::Dark(BaseColor::Black), theme::Color::Dark(BaseColor::Black)),
        }
    }
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rotation {
    R0 = 0,
    R90 = 1,
    R180 = 2,
    R270 = 3,
}