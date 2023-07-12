use crate::board::Board;
use crate::queue::Queue;
use crate::score::Score;

use cursive::{
    theme::{ Color, ColorStyle,  },
    Vec2,
};
use wasm_bindgen::prelude::*;
use web_sys::console;

const SLOW_SPEED: usize = 30;
const NORMAL_SPEED: usize = 10;
const FAST_SPEED: usize = 1;

#[wasm_bindgen]
pub struct Tetris {
    board: Board,
    queue: Queue,
    score: Score,
    board_size: Vec2,
    score_size: Vec2,
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
        let mut board = Board::new(10, 20);
        let board_size = board.required_size(Vec2::new(0,0));
        let mut score = Score::new();
        let score_size = score.required_size(Vec2::new(0,0));

        Tetris {
            board,
            queue: Queue::new(),
            score,
            board_size,
            score_size,
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
        self.score.renew();
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
        let gameover = gameover || self.score.is_gameover();
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

    fn merge_block(&mut self) {
        // let score = self.board.merge_block();
        let score = 5;
        self.score.add(score);
        // let block = self.queue.pop_and_spawn_new_block();
        // self.board.insert(block);
        self.hit_bottom = false;
        self.max_frame_idx = SLOW_SPEED;
        self.frame_idx = 0;
    }

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
        console::log_1(&format!("tetris printer offset: {:?}", printer.offset).into());
        let mut printer = printer.clone();
        printer.offset = Vec2::new(0,0);

        let size = Vec2::new(1000,1000);
        for j in 0..size.y {
            for i in 0..size.x {
                printer.with_color(ColorStyle::new(Color::Rgb(5,247,13), Color::Rgb(5,247,13)), |printer| {
                    printer.print((i, j), " ");
                });
            }
        }

        let x_padding = 5;
        let y_padding = 2;
        
        let score_padding = Vec2::new(x_padding, y_padding);
        let score_printer = printer.offset(score_padding);

        let board_padding = Vec2::new(x_padding + self.score_size.x + x_padding, y_padding);
        let board_printer = printer.offset(board_padding);

        let queue_padding = Vec2::new(x_padding + self.score_size.x + x_padding + self.board_size.x + x_padding, y_padding);
        let queue_printer = printer.offset(queue_padding);

        self.score.draw(&score_printer);
        self.board.draw(&board_printer);
        self.queue.draw(&queue_printer);

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

    fn required_size(&mut self, _constraint: cursive::Vec2) -> cursive::Vec2 {
        Vec2::new(1000, 1000)

    }
}