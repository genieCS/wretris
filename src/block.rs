use crate::pos::Pos;

use wasm_bindgen::prelude::*;
use cursive::theme::{
    BaseColor, ColorStyle, self,
};
use rand::thread_rng;
use rand::seq::SliceRandom;



#[derive(Clone, Debug)]
pub struct BlockWithPos {
    pub block: Block,
    pub pos: Pos,
}

impl BlockWithPos {
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

    pub fn color(&self) -> BColor {
        self.block.color()
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
                shape: self.shape,
                rotation: rotation.flip_turn(),
            },
        }
    }

    pub fn rotate(&self, clockwise: bool) -> Self {
        match (&self.shape, &self.rotation) {
            (Shape::O, _) => self.clone(),
            (_, rotation) => Block {
                shape: self.shape,
                rotation: if clockwise { rotation.clockwise() } else { rotation.counter_clockwise() },
            },
        }
    }

    pub fn color(&self) -> BColor {
        self.shape.to_color()
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

    fn to_color(&self) -> BColor {
        match self {
            Shape::I => BColor::I,
            Shape::O => BColor::O,
            Shape::T => BColor::T,
            Shape::S => BColor::S,
            Shape::Z => BColor::Z,
            Shape::J => BColor::J,
            Shape::L => BColor::L,
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
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BColor {
    I = 0,
    O = 1,
    T = 2,
    S = 3,
    Z = 4,
    J = 5,
    L = 6,
    GRID1 = 7,
    GRID2 = 8,
    HINT = 9,
    WARNING = 10,
}

impl BColor {
    pub fn to_cursive(&self) -> ColorStyle {
        match self {
            BColor::I => ColorStyle::new(theme::Color::Light(BaseColor::Blue), theme::Color::Light(BaseColor::Blue)),
            BColor::O => ColorStyle::new(theme::Color::Light(BaseColor::Yellow), theme::Color::Light(BaseColor::Yellow)),
            BColor::T => ColorStyle::new(theme::Color::Light(BaseColor::Magenta), theme::Color::Light(BaseColor::Magenta)),
            BColor::S => ColorStyle::new(theme::Color::Light(BaseColor::Green), theme::Color::Light(BaseColor::Green)),
            BColor::Z => ColorStyle::new(theme::Color::Light(BaseColor::Red), theme::Color::Light(BaseColor::Red)),
            BColor::J => ColorStyle::new(theme::Color::Light(BaseColor::Cyan), theme::Color::Light(BaseColor::Cyan)),
            BColor::L => ColorStyle::new(theme::Color::Light(BaseColor::White), theme::Color::Light(BaseColor::White)),
            BColor::GRID1 => ColorStyle::new(theme::Color::Rgb(0xb2, 0xe6, 0xc6), theme::Color::Rgb(0xb2, 0xe6, 0xc6)),
            BColor::GRID2 => ColorStyle::new(theme::Color::Rgb(0x08, 0x95, 0x62), theme::Color::Rgb(0x08, 0x95, 0x62)),
            BColor::HINT => ColorStyle::new(theme::Color::Dark(BaseColor::White), theme::Color::Dark(BaseColor::White)),
            BColor::WARNING => ColorStyle::new(theme::Color::Light(BaseColor::Yellow), theme::Color::Light(BaseColor::Yellow)),

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