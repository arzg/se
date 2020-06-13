#![warn(rust_2018_idioms)]

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn process_keypress(key_event: KeyEvent) -> ControlFlow {
    let KeyEvent { code, modifiers } = key_event;

    if code == KeyCode::Char('q') && modifiers == KeyModifiers::CONTROL {
        return ControlFlow::Break;
    }

    if let KeyCode::Char(c) = code {
        let mut buf = [0; 4];
        c.encode_utf8(&mut buf);

        println!("{:?} ('{}')\r", buf, c);
    }

    ControlFlow::Continue
}

pub enum ControlFlow {
    Continue,
    Break,
}
