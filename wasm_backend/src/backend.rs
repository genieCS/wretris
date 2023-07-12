use cursive_core::{
    event::{Event},
    Vec2,
    theme,
};
use std::collections::VecDeque;
use std::rc::Rc;
use std::cell::RefCell;
use web_sys::{
    HtmlCanvasElement,
    CanvasRenderingContext2d,
    console,
};
use wasm_bindgen::prelude::*;
use crate::theme::{ColorPair, cursive_to_color_pair, cursive_to_color, };


pub struct Backend {
    canvas: HtmlCanvasElement,
    ctx: CanvasRenderingContext2d,
    color: RefCell<ColorPair>,
    font_height: usize,
    font_width: usize,
    events: Rc<RefCell<VecDeque<Event>>>,
}

impl Backend {
    pub fn init(canvas: HtmlCanvasElement) -> Self {
        canvas.set_width(1000);
        canvas.set_height(1000);
        let color = RefCell::new(cursive_to_color_pair(theme::ColorPair {
            front: theme::Color::Light(theme::BaseColor::White),
            back: theme::Color::Dark(theme::BaseColor::Black),
        }));

        let font_width = 12;     
        let font_height = font_width * 2;
        let ctx: CanvasRenderingContext2d = canvas.get_context("2d").unwrap().unwrap().dyn_into().unwrap();
        ctx.set_font(&format!("{}px monospace", 20));

        let events = Rc::new(RefCell::new(VecDeque::new()));
         let cloned = events.clone();
         let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
             for c in event.key().chars() {
                cloned.borrow_mut().push_back(Event::Char(c));
             }
         }) as Box<dyn FnMut(_)>);
         canvas.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref()).unwrap();
         closure.forget();

        Backend { 
            canvas,
            ctx,
            color,
            font_height,
            font_width,
            events,     
         }
    }
}

impl cursive_core::backend::Backend for Backend {
    fn poll_event(self: &mut Backend) -> Option<Event> {
        self.events.borrow_mut().pop_front()
    }

    fn set_title(self: &mut Backend, title: String) {
        self.canvas.set_title(&title);
    }

    fn refresh(self: &mut Backend) {
        console::log_1(&"refresh".into());
    }

    fn has_colors(self: &Backend) -> bool {
        true
    }

    fn screen_size(self: &Backend) -> Vec2 {
        Vec2::new(self.canvas.width() as usize, self.canvas.height() as usize)
    }

    fn print_at(self: &Backend, pos: Vec2, text: &str) {
        // if self.color.borrow().back != cursive_to_color(theme::Color::Dark(theme::BaseColor::Blue)) {
        //     console::log_1(&JsValue::from_str(&format!("color: {} pos: {:?}, length: {}, text: {}",self.color.borrow().back, pos, text.len(), text)));
        // }
        let color = self.color.borrow();
        self.ctx.set_fill_style(&JsValue::from_str(&color.back));
        // if self.color.borrow().back != "#c0c0c0" && self.color.borrow().back != "#000080" {
        if self.color.borrow().back != "#000080" { // not blue
            self.ctx.fill_rect((pos.x * self.font_width) as f64, (pos.y * self.font_height) as f64, (self.font_width * text.len()) as f64, self.font_height as f64);
        } else { // blue
            // console::log_1(&JsValue::from_str(&format!("else pos {:?}", pos)));
            self.ctx.fill_rect((pos.x * self.font_width) as f64, (pos.y * self.font_height) as f64, (self.font_width * text.len()) as f64, self.font_height as f64);
        }
        self.ctx.set_fill_style(&JsValue::from_str(&color.front));
        self.ctx.fill_text(text, (pos.x * self.font_width) as f64, (pos.y * self.font_height + self.font_height * 3/4) as f64).unwrap();
    }

    fn clear(self: &Backend, color: cursive_core::theme::Color) {
        self.ctx.set_fill_style(&JsValue::from_str(&cursive_to_color(color)));
    }

    fn set_color(self: &Backend, color_pair: cursive_core::theme::ColorPair) -> cursive_core::theme::ColorPair {
        // if color_pair.back != cursive_core::theme::Color::Dark(cursive_core::theme::BaseColor::Blue) {
            // console::log_1(&JsValue::from_str(&format!("set color not for background: {:?}", color_pair)));
        // }
        // console::log_1(&JsValue::from_str(&format!("set color: {:?}", color_pair)));
        if self.color.borrow().front == cursive_to_color(color_pair.front) && self.color.borrow().back == cursive_to_color(color_pair.back) {
            return color_pair;
        }
        let mut color = self.color.borrow_mut();
        *color = cursive_to_color_pair(color_pair);
        color_pair
    }

    fn set_effect(self: &Backend, _: cursive_core::theme::Effect) {
    }

    fn unset_effect(self: &Backend, _: cursive_core::theme::Effect) {
    }

    fn name(&self) -> &str {
        "cursive-wasm-backend"
    }
}