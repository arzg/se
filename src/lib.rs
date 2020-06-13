#![warn(rust_2018_idioms)]

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn process_keypress(key_event: KeyEvent) -> ControlFlow {
    let KeyEvent { code, modifiers } = key_event;

    if code == KeyCode::Char('q') && modifiers == KeyModifiers::CONTROL {
        ControlFlow::Break
    } else {
        ControlFlow::Continue
    }
}

pub enum ControlFlow {
    Continue,
    Break,
}
