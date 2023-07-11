use crate::block::{ BlockWithPos, Color, Pos, };
use wasm_bindgen::prelude::*;
use cursive::{
    event::{Callback, Event, EventResult, Key},
    Printer,
    View,
    views::Dialog,
};
use web_sys::console;

const SLOW_SPEED: usize = 30;
const NORMAL_SPEED: usize = 10;
const FAST_SPEED: usize = 1;

#[derive(Clone, Copy)]
enum LRD {
    Left,
    Right,
    Down,
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

#[wasm_bindgen]
pub struct ColorGrid {
    width: usize,
    height: usize,
    data: Vec<Color>,
    block: BlockWithPos,
    hit_bottom: bool,
    background_color: Color,
    gameover: bool,
    max_frame_idx: usize,
    frame_idx: usize,
    is_paused: bool,
}


#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
impl ColorGrid {
    pub fn new() -> ColorGrid {
        ColorGrid {
            width: 10,
            height: 20,
            data: vec![Color::EMPTY; 10 * 20],
            block: BlockWithPos::new(),
            hit_bottom: false,
            background_color: Color::EMPTY,
            gameover: false,
            max_frame_idx: NORMAL_SPEED,
            frame_idx: 0,
            is_paused: false,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    fn handle_merge_and_pass(&mut self, event: Event) -> EventResult {
        if self.gameover && event != Event::Char('n') {
            return EventResult::Consumed(None);
        }
        let is_begin = self.hit_bottom;
        if self.hit_bottom {
            self.merge_block();
        }
        match event {
            Event::Key(Key::Down) => self.speed_up(),
            Event::Refresh => self.on_down_wrapper(false, is_begin),
            Event::Char(' ') => self.on_down_wrapper(true, is_begin),
            Event::Char('n') => self.new_game(),
            Event::Char('m') => self.stop_and_resume(),
            _ => EventResult::Ignored,
        }
    }

    fn speed_up(&mut self) -> EventResult {
        self.max_frame_idx = FAST_SPEED;
        self.frame_idx = 0;
        EventResult::Consumed(None)
    }

    fn on_down_wrapper(&mut self, is_drop: bool, is_begin: bool) -> EventResult {
        if self.is_paused {
            return EventResult::Consumed(None);
        }
        let (gameover, hit_bottom) = self.on_down(is_drop, is_begin);
        let gameover = gameover;
        if gameover {
            self.gameover = true;
            self.toggle_pause();
            return EventResult::Consumed(Some(Callback::from_fn(move |s| {
                s.add_layer(Dialog::info("Game Over!"));
            })));
        }
        if hit_bottom {
            if is_drop {
                self.merge_block();
            } else {
                self.hit_bottom = hit_bottom;
                self.frame_idx = 0;
                self.max_frame_idx = NORMAL_SPEED;
            }
        }
        EventResult::Consumed(None)
    }

    fn new_game(&self) -> EventResult {
        EventResult::Consumed(None)
    }

    fn stop_and_resume(&self) -> EventResult {
        EventResult::Consumed(None)
    }

    fn toggle_pause(&self) {}

    fn on_down(&mut self, is_drop: bool, is_begin: bool) -> (bool, bool) {
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

    fn pass_event_to_board(&self) -> EventResult {
        EventResult::Consumed(None)
    }
}

impl ColorGrid {
    fn draw_background(&self, printer: &Printer) {
        let width = self.width();
        let height = self.height();
        for i in 0..width {
            for j in 0..height {
                printer.with_color(self.data[width * j + i].to_cursive(), |printer| {
                    printer.print((2*i, j), "  ");
                });
            }
        }
    }

    fn draw_block(&self, printer: &Printer) {
        let width = self.width();
        let height = self.height();
        let pos = self.block.pos;
        for (i, j) in self.block.cells() {
            let x = pos.0 + i;
            let y = pos.1 + j;
            if x < 0 || x >= width as i32 || y < 0 || y >= height as i32 {
                continue;
            }
            printer.with_color(self.block.to_cursive_color(), |printer| {
                printer.print((2*x as usize, y as usize), "  ");
            });
        }
    }
}

impl View for ColorGrid {
    fn draw(&self, printer: &Printer) {
        console::log_1(&JsValue::from_str(&"draw ColorGrid"));
        self.draw_background(printer);
        self.draw_block(printer);
    }

    fn required_size(&mut self, _constraint: cursive::Vec2) -> cursive::Vec2 {
        let width = self.width();
        let height = self.height();
        cursive::Vec2::new(10*width + 3,  10 * height + 10)
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Refresh | Event::Key(Key::Down) | Event::Char(' ') | Event::Char('n') => self.handle_merge_and_pass(event),
            _ => self.pass_event_to_board(),

        }
    }
}