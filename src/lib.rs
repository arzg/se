#![warn(rust_2018_idioms)]

use crossterm::{cursor, event, queue, terminal};
use std::io::{self, Write};

pub struct Editor {
    screen_rows: u16,
    screen_cols: u16,
}

impl Editor {
    pub fn new() -> anyhow::Result<Self> {
        let (screen_cols, screen_rows) = terminal::size()?;

        Ok(Self {
            screen_rows,
            screen_cols,
        })
    }

    pub fn refresh_screen(&self, stdout: &mut io::Stdout) -> anyhow::Result<()> {
        queue!(
            stdout,
            cursor::Hide,
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0)
        )?;

        self.draw_rows(stdout)?;

        queue!(stdout, cursor::MoveTo(0, 0), cursor::Show)?;

        stdout.flush()?;

        Ok(())
    }

    fn draw_rows(&self, stdout: &mut io::Stdout) -> anyhow::Result<()> {
        let is_on_last_row = |i| i == self.screen_rows - 1;

        for i in 0..self.screen_rows {
            write!(stdout, "~")?;

            if !is_on_last_row(i) {
                writeln!(stdout, "\r")?;
            }
        }

        Ok(())
    }

    pub fn process_keypress(&self, key_event: event::KeyEvent) -> ControlFlow {
        let event::KeyEvent { code, modifiers } = key_event;

        if code == event::KeyCode::Char('q') && modifiers == event::KeyModifiers::CONTROL {
            ControlFlow::Break
        } else {
            ControlFlow::Continue
        }
    }
}

pub enum ControlFlow {
    Continue,
    Break,
}
