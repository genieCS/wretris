use crate::pos::Pos;

#[derive(Clone, Copy)]
pub enum LR {
    Left,
    Right,
}

impl LR {
    pub fn to_lrd(&self) -> LRD {
        match self {
            LR::Left => LRD::Left,
            LR::Right => LRD::Right,
        }
    }
}

#[derive(Clone, Copy)]
pub enum LRD {
    Left,
    Right,
    Down,
}

impl LRD {
    pub fn delta(&self) -> Pos {
        match self {
            LRD::Left => (-1, 0),
            LRD::Right => (1, 0),
            LRD::Down => (0, 1),
        }
    }
}