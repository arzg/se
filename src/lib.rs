#![warn(rust_2018_idioms)]

use crossterm::{cursor, event, queue, terminal};
use std::io::{self, Write};

pub fn refresh_screen(stdout: &mut io::Stdout) -> anyhow::Result<()> {
    queue!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    )?;

    draw_rows(stdout)?;

    queue!(stdout, cursor::MoveTo(0, 0))?;

    stdout.flush()?;

    Ok(())
}

fn draw_rows(stdout: &mut io::Stdout) -> anyhow::Result<()> {
    // We don’t know how high the terminal is at the moment, so we just default to 24 characters.
    for _ in 0..24 {
        writeln!(stdout, "~\r")?;
    }

    Ok(())
}

pub fn process_keypress(key_event: event::KeyEvent) -> ControlFlow {
    let event::KeyEvent { code, modifiers } = key_event;

    if code == event::KeyCode::Char('q') && modifiers == event::KeyModifiers::CONTROL {
        ControlFlow::Break
    } else {
        ControlFlow::Continue
    }
}

pub enum ControlFlow {
    Continue,
    Break,
}
