#![warn(rust_2018_idioms)]

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::{cursor, execute, terminal};
use std::io::{self, Write};

pub fn refresh_screen(stdout: &mut io::Stdout) -> anyhow::Result<()> {
    execute!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    )?;

    Ok(())
}

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
