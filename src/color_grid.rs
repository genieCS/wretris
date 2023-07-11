use crate::block::{ BlockWithPos, Color, Pos, };
use std::ops::Index;

#[derive(Clone, Copy)]
enum LRD {
    Left,
    Right,
    Down,
}

#[derive(Clone, Copy)]
pub enum LR {
    Left,
    Right,
}

impl LR {
    fn to_lrd(&self) -> LRD {
        match self {
            LR::Left => LRD::Left,
            LR::Right => LRD::Right,
        }
    }
}

impl LRD {
    fn delta(&self) -> Pos {
        match self {
            LRD::Left => (-1, 0),
            LRD::Right => (1, 0),
            LRD::Down => (0, 1),
        }
    }
}

pub struct ColorGrid {
    width: usize,
    height: usize,
    data: Vec<Color>,
    pub block: BlockWithPos,
    background_color: Color,
}

impl ColorGrid {
    pub fn new(width: usize, height: usize) -> ColorGrid {
        ColorGrid {
            width,
            height,
            data: vec![Color::O; width * height],
            block: BlockWithPos::new(),
            background_color: Color::O,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn handle_lr(&mut self, lr: LR, hit_bottom: bool, is_hard: bool) -> bool {
        let lrd = lr.to_lrd();
        let mut stopped = false;
        let mut moved = false;
        while !stopped {
            let (block, hit_wall) = self.move_block_lrd(lrd);
            if block.is_some() {
                moved = true;
                self.block = block.unwrap();
            }
            stopped = hit_wall || !is_hard;
        }
        if hit_bottom {
            self.on_down(true, false);
        }
        moved
    }

    pub fn on_down(&mut self, is_drop: bool, is_begin: bool) -> (bool, bool) {
        let mut stopped = false;
        let mut hit_bottom = is_drop;
        let mut current: Option<BlockWithPos>;
        let gameover = false;
        while !stopped {
           (current, hit_bottom)= self.move_block_lrd(LRD::Down);
            match current {
                Some(b) => self.block = b,
                None => return (is_begin, true),
            }
            stopped = hit_bottom || !is_drop;
        }
        (gameover, hit_bottom)
    }

    fn move_block_lrd(&mut self, lrd: LRD) -> (Option<BlockWithPos>, bool) {
        let block = &self.block;
        let (can_move, stop) = self.can_move(block, lrd);
        if !can_move {
            return (None, stop)
        }
        let delta = lrd.delta();
        let x = block.pos.0 as i32 + delta.0;
        let y = block.pos.1 as i32 + delta.1;
        let bwp = BlockWithPos::from(block.block.clone(), (x, y));
        (Some(bwp), stop)
    }

    fn can_move(&self, block: &BlockWithPos, lrd: LRD) -> (bool, bool) {
        let delta = lrd.delta();
        let mut moved = true;
        let mut stop = false;
        let width = self.width();
        let height = self.height();
        for (x, y) in block.cells() {
            let next_x = x + delta.0;
            let next_y = y + delta.1;
            if next_x < 0 || next_x >= width as i32 || next_y < 0 || next_y >= height as i32 || self.is_occupied(next_x, next_y) {
                moved = false;
                stop = true;
                break;
            } else if next_y + 1 == height as i32 || self.is_occupied(next_x, next_y + 1){
                stop = true;
            }
        }
        (moved, stop)
    }

    fn is_occupied(&self, x: i32, y: i32) -> bool {
        self.data[self.width() *  y as usize + x as usize] != self.background_color
    }

    fn merge_block(&mut self) {}
}

impl Index<usize> for ColorGrid {
    type Output = Color;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
