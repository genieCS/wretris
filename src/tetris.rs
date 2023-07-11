use wasm_bindgen::prelude::*;
use crate::board::Board;

const SLOW_SPEED: usize = 30;
const NORMAL_SPEED: usize = 10;
const FAST_SPEED: usize = 1;

#[wasm_bindgen]
pub struct Tetris {
    board: Board,
    is_paused: bool,
    hit_bottom: bool,
    frame_idx: usize,
    max_frame_idx: usize,
    gameover: bool,
}

use cursive::{
    event::{Callback, Event, EventResult, Key},
    Printer, View,
    views::Dialog,
};

impl Default for Tetris {
    fn default() -> Self {
        Self::new()
    }
}

impl Tetris {
    pub fn new() -> Self {
        Tetris {
            board: Board::new(10, 20),
            is_paused: false,
            hit_bottom: false,
            frame_idx: 0,
            max_frame_idx: SLOW_SPEED,
            gameover: false,
        }
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

            Event::Char('n') => self.new_game(),
            Event::Char('m') => self.stop_and_resume(),
            Event::Refresh => self.on_down(false, is_begin),
            Event::Char(' ') => self.on_down(true, is_begin),
            _ => EventResult::Ignored,
        }
    }

    fn speed_up(&mut self) -> EventResult {
        self.max_frame_idx = FAST_SPEED;
        self.frame_idx = 0;
        EventResult::Consumed(None)
    }

    fn new_game(&mut self) -> EventResult {
        self.board.renew();
        self.is_paused = false;
        self.hit_bottom = false;
        self.frame_idx = 0;
        self.max_frame_idx = SLOW_SPEED;
        self.gameover = false;
        EventResult::Consumed(None)
    }

    fn stop_and_resume(&mut self) -> EventResult {
        self.toggle_pause();
        if self.is_paused {
            EventResult::Consumed(Some(Callback::from_fn(move |s| {
                s.add_layer(Dialog::info("Game Over!"));
            })))
        } else {
            EventResult::Consumed(None)
        }
    }

    fn toggle_pause(&mut self) {
        self.is_paused = !self.is_paused;
    }

    fn on_down(&mut self, is_drop: bool, is_begin: bool) -> EventResult {
        if self.is_paused {
            return EventResult::Consumed(None);
        }
        let (gameover, hit_bottom) = self.board.on_down(is_drop, is_begin);
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

    fn merge_block(&self) {}

    fn pass_event_to_board(&mut self, event: Event) -> EventResult {
        if self.is_paused || self.gameover {
            return EventResult::Consumed(None)
        }
        let moved = self.board.handle_event(event, self.hit_bottom);
        if self.hit_bottom && moved {
            self.max_frame_idx = std::cmp::min(3 + self.max_frame_idx, 2 * NORMAL_SPEED);
        }
        EventResult::Consumed(None)
    }
}

impl View for Tetris {
    fn draw(&self, printer: &Printer) {
        self.board.draw(printer);
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        if event == Event::Refresh {
            self.frame_idx += 1;
            if self.frame_idx == self.max_frame_idx {
                self.frame_idx = 0;
            } else {
                return EventResult::Ignored;
            }
        }

        match event {
            Event::Refresh | Event::Key(Key::Down) | Event::Char(' ') | Event::Char('n') | Event::Char('m') => self.handle_merge_and_pass(event),
            _ => self.pass_event_to_board(event),
        }
    }

    fn required_size(&mut self, constraint: cursive::Vec2) -> cursive::Vec2 {
        self.board.required_size(constraint)
    }
}