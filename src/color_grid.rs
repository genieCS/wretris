use crate::block::{ Block, BlockWithPos, Color, };
use crate::lrd::{ LRD, LR };

use std::ops::Index;

#[derive(Clone, Copy)]
enum FlipRotate {
    FlipTurn,
    Rotate {
        clockwise: bool,
    }
}

pub struct ColorGrid {
    pub width: usize,
    pub height: usize,
    data: Vec<Color>,
    pub block: BlockWithPos,
    background_color: (Color, Color),
}

impl ColorGrid {
    pub fn new(width: usize, height: usize, background_color: (Color, Color)) -> ColorGrid {
        let mut data = Vec::with_capacity(width * height);
        for h in 0..height {
            for w in 0..width {
                    let color = if (w + h) % 2 == 0 {
                        background_color.0
                    } else {
                        background_color.1
                    };
                    data.push(color);
            }
        }
        ColorGrid {
            width,
            height,
            data,
            block: Self::insert_random(width),
            background_color,
        }
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

    pub fn rotate(&mut self, hit_bottom: bool, clockwise: bool) -> bool {
        self.flip_rotate(hit_bottom, FlipRotate::Rotate { clockwise })
    }

    pub fn flip_turn(&mut self, hit_bottom: bool) -> bool {
        self.flip_rotate(hit_bottom, FlipRotate::FlipTurn)
    }

    fn flip_rotate(&mut self, hit_bottom: bool, flip_rotate: FlipRotate) -> bool {
        for cell in self.block.cells() {
            let mut pos = cell;
            for _ in 0..10 {
                let mut possible = true;
                let next_block = match flip_rotate {
                    FlipRotate::FlipTurn => self.block.flip_turn(),
                    FlipRotate::Rotate { clockwise } => self.block.rotate(clockwise),
                };

                for (x, y) in next_block.cells() {
                    if x < 0 {
                        pos.0 += 1;
                        possible = false;
                        break;
                    } else if x >= self.width as i32 {
                        pos.0 -= 1;
                        possible = false;
                        break;
                    } else if y < 0 {
                        pos.1 += 1;
                        possible = false;
                        break;
                    } else if y >= self.height as i32 {
                        pos.1 -= 1;
                        possible = false;
                        break;
                    } else if self.is_occupied(x as usize, y as usize) {
                        possible = false;
                        break;
                    }
                }
                if possible {
                    self.block = BlockWithPos::from(next_block.block, pos);
                    if hit_bottom {
                        self.on_down(true, false);
                    }
                    return true
                }
            }
        }
        false
    }

    fn move_block_lrd(&mut self, lrd: LRD) -> (Option<BlockWithPos>, bool) {
        let block = &self.block;
        let (can_move, stop) = self.can_move(block, lrd);
        if !can_move {
            return (None, stop)
        }
        let delta = lrd.delta();
        let x = block.pos.0 + delta.0;
        let y = block.pos.1 + delta.1;
        let bwp = BlockWithPos::from(block.block.clone(), (x, y));
        (Some(bwp), stop)
    }

    fn can_move(&self, block: &BlockWithPos, lrd: LRD) -> (bool, bool) {
        let delta = lrd.delta();
        let mut moved = true;
        let mut stop = false;
        for (x, y) in block.cells() {
            let next_x = x + delta.0;
            let next_y = y + delta.1;
            if next_x < 0 || next_x >= self.width as i32 || next_y < 0 || next_y >= self.height as i32 || self.is_occupied(next_x as usize, next_y as usize) {
                moved = false;
                stop = true;
                break;
            } else if next_y + 1 == self.height as i32 || self.is_occupied(next_x as usize, next_y as usize + 1){
                stop = true;
            }
        }
        (moved, stop)
    }

    fn is_occupied(&self, x: usize, y: usize) -> bool {
        self.data[self.width * y  + x] != self.background_color.0 && self.data[self.width * y  + x] != self.background_color.1
    }

    pub fn merge_block(&mut self) -> usize {
        self.fill_board_with_block();
        self.remove_rows_if_possible()
    }

    pub fn insert(&mut self, block: Block) {
        self.block = BlockWithPos::from(block, (self.width as i32 / 2, 1));
    }

    fn fill_board_with_block(&mut self) {
        for cell in self.block.cells() {
            self.data[self.width * cell.1 as usize + cell.0 as usize] = self.block.color();
        }
    }

    fn remove_rows_if_possible(&mut self) -> usize {
        let mut rows_to_remove = Vec::new();
        for _y in 0..self.height {
            let y = self.height - _y - 1;
            let mut remove = true;
            for x in 0..self.width {
                if !self.is_occupied(x, y) {
                    remove = false;
                    break;
                }
            }
            if remove {
                rows_to_remove.push(y);
            }
        }
        let score = rows_to_remove.len();
        self.remove_rows(rows_to_remove);
        score
    }

    fn remove_rows(&mut self, rows_to_remove: Vec<usize>) {
        if rows_to_remove.is_empty() {
            return;
        }
        let mut fill_y = self.height - 1;
        let mut check_y = self.height - 1;
        for row in rows_to_remove {
            while check_y > row {
                if fill_y != check_y {
                    self.set_background_row(check_y, fill_y);
                }
                fill_y -= 1;
                check_y -= 1;
            }
            check_y = row - 1;
        }
        while check_y > 0 {
            self.set_background_row(check_y, fill_y);
            fill_y -= 1;
            check_y -= 1;
        }
        while fill_y > 0 {
            for x in 0..self.width {
                self.set_background(x, fill_y);
            }
            fill_y -= 1;
        }
    }

    fn set_background(&mut self, x: usize, y: usize) {
        let color = if (x + y) % 2 == 0 {
            self.background_color.0
        } else {
            self.background_color.1
        };
        self.data[self.width * y  + x] = color;
    }

    pub fn renew(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.set_background(x, y);
            }
        }
        self.block = ColorGrid::insert_random(self.width)
    }

    fn set_background_row(&mut self, from: usize, to: usize) {
        for w in 0..self.width {
            if self.is_occupied(w, from) {
                self.data[self.width * to + w] = self.data[self.width * from + w];
                continue
            }
            let color = if (w + to) % 2 == 0 {
                self.background_color.0
            } else {
                self.background_color.1
            };
            self.data[self.width * to + w] = color;        
        }
    }

    pub fn insert_random(width: usize) -> BlockWithPos {
        BlockWithPos::from(Block::default(), (width as i32 / 2, 1))
    }
}

impl Index<usize> for ColorGrid {
    type Output = Color;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
