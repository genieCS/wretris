use crate::pos::Pos;

use wasm_bindgen::prelude::*;
use cursive::{
    theme::{ BaseColor, ColorStyle, self },
};
use rand::thread_rng;
use rand::seq::SliceRandom;



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

    pub fn from(block: Block, pos: Pos) -> Self {
        BlockWithPos {
            block,
            pos,
        }
    }

    pub fn to_cursive_color(&self) -> ColorStyle {
        self.block.shape.to_cursive()
    }

    pub fn cells(&self) -> Vec<Pos> {
        self.block.cells().into_iter().map(|(x,y)| (x + self.pos.0, y + self.pos.1)).collect()
    }

    pub fn flip_turn(&self) -> BlockWithPos {
        Self {
            block: self.block.flip_turn(),
            pos: self.pos,
        }
    }

    pub fn rotate(&self, clockwise: bool) -> BlockWithPos {
        Self {
            block: self.block.rotate(clockwise),
            pos: self.pos,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Block 
{
    shape: Shape,
    rotation: Rotation,
}

impl Default for Block {
    fn default() -> Self {
        Self {
            shape: Shape::random(),
            rotation: Rotation::R0,
        }
    }
}

impl Block {
    pub fn new(shape: Shape) -> Self {
        Self {
            shape,
            rotation: Rotation::R0,
        }
    }

    pub fn cells(&self) -> Vec<Pos> {
        match self.rotation {
            Rotation::R0 => self.shape.cells(),
            Rotation::R90 => self.shape.cells().into_iter().map(|(x,y)| (-y,x)).collect(),
            Rotation::R180 => self.shape.cells().into_iter().map(|(x,y)| (-x,-y)).collect(),
            Rotation::R270 => self.shape.cells().into_iter().map(|(x,y)| (y,-x)).collect(),
        }
    }

    pub fn flip_turn(&self) -> Self {
        match (&self.shape, &self.rotation) {
            (Shape::O, _) => self.clone(),
            (_, rotation) => Block {
                shape: self.shape.clone(),
                rotation: rotation.flip_turn(),
            },
        }
    }

    pub fn rotate(&self, clockwise: bool) -> Self {
        match (&self.shape, &self.rotation) {
            (Shape::O, _) => self.clone(),
            (_, rotation) => Block {
                shape: self.shape.clone(),
                rotation: if clockwise { rotation.clockwise() } else { rotation.counter_clockwise() },
            },
        }
    }

    pub fn color(&self) -> ColorStyle {
        self.shape.to_color().to_cursive()
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
    fn random() -> Self {
        Self::all().pop().unwrap()
    }

    pub fn all() -> Vec<Shape> {
        let mut shapes = vec![Shape::I, Shape::O, Shape::T, Shape::S, Shape::Z, Shape::J, Shape::L];
        shapes.shuffle(&mut thread_rng());
        shapes
    }


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
            Color::I => ColorStyle::new(theme::Color::Light(BaseColor::Blue), theme::Color::Light(BaseColor::Blue)),
            Color::O => ColorStyle::new(theme::Color::Light(BaseColor::Yellow), theme::Color::Light(BaseColor::Yellow)),
            Color::T => ColorStyle::new(theme::Color::Light(BaseColor::Magenta), theme::Color::Light(BaseColor::Magenta)),
            Color::S => ColorStyle::new(theme::Color::Light(BaseColor::Green), theme::Color::Light(BaseColor::Green)),
            Color::Z => ColorStyle::new(theme::Color::Light(BaseColor::Red), theme::Color::Light(BaseColor::Red)),
            Color::J => ColorStyle::new(theme::Color::Light(BaseColor::Cyan), theme::Color::Light(BaseColor::Cyan)),
            Color::L => ColorStyle::new(theme::Color::Light(BaseColor::White), theme::Color::Light(BaseColor::White)),
            Color::EMPTY => ColorStyle::new(theme::Color::Dark(BaseColor::Blue), theme::Color::Dark(BaseColor::Blue)),
            Color::HINT => ColorStyle::new(theme::Color::Dark(BaseColor::White), theme::Color::Dark(BaseColor::White)),
            Color::WARNING => ColorStyle::new(theme::Color::Light(BaseColor::Yellow), theme::Color::Light(BaseColor::Yellow)),

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

impl Rotation {
    pub fn flip_turn(&self) -> Self {
        match self {
            Rotation::R0 => Rotation::R180,
            Rotation::R90 => Rotation::R270,
            Rotation::R180 => Rotation::R0,
            Rotation::R270 => Rotation::R90,
        }
    }

    pub fn clockwise(&self) -> Self {
        match self {
            Rotation::R0 => Rotation::R90,
            Rotation::R90 => Rotation::R180,
            Rotation::R180 => Rotation::R270,
            Rotation::R270 => Rotation::R0,
        }
    }

    pub fn counter_clockwise(&self) -> Self {
        match self {
            Rotation::R0 => Rotation::R270,
            Rotation::R90 => Rotation::R0,
            Rotation::R180 => Rotation::R90,
            Rotation::R270 => Rotation::R180,
        }
    }
}