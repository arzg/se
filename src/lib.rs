#![warn(rust_2018_idioms)]

use crossterm::{cursor, event, queue, terminal};
use std::convert::TryFrom;
use std::io::{self, Write};

const WELCOME_MSG: &str = concat!("se v", env!("CARGO_PKG_VERSION"), " · A screen editor.");

pub struct Editor {
    cursor_x: u16,
    cursor_y: u16,
    screen_rows: u16,
    screen_cols: u16,
}

impl Editor {
    pub fn new() -> anyhow::Result<Self> {
        let (screen_cols, screen_rows) = terminal::size()?;

        Ok(Self {
            cursor_x: 0,
            cursor_y: 0,
            screen_rows,
            screen_cols,
        })
    }

    pub fn refresh_screen(&self, stdout: &mut io::Stdout) -> anyhow::Result<()> {
        queue!(stdout, cursor::Hide, cursor::MoveTo(0, 0))?;

        self.draw_rows(stdout)?;

        queue!(
            stdout,
            cursor::MoveTo(self.cursor_x, self.cursor_y),
            cursor::Show
        )?;

        stdout.flush()?;

        Ok(())
    }

    fn draw_rows(&self, stdout: &mut io::Stdout) -> anyhow::Result<()> {
        let is_on_last_row = |i| i == self.screen_rows - 1;

        for i in 0..self.screen_rows {
            write!(stdout, "~")?;

            if i == self.screen_rows / 3 {
                let padding_len =
                    (usize::try_from(self.screen_cols).unwrap() - WELCOME_MSG.len()) / 2;

                let padding = " ".repeat(padding_len);

                write!(stdout, "{}{}", padding, WELCOME_MSG)?;
            }

            queue!(stdout, terminal::Clear(terminal::ClearType::UntilNewLine))?;

            if !is_on_last_row(i) {
                writeln!(stdout, "\r")?;
            }
        }

        Ok(())
    }

    pub fn process_keypress(&mut self, key_event: event::KeyEvent) -> ControlFlow {
        let event::KeyEvent { code, modifiers } = key_event;

        if code == event::KeyCode::Char('q') && modifiers == event::KeyModifiers::CONTROL {
            return ControlFlow::Break;
        }

        match code {
            event::KeyCode::Left => self.cursor_x = self.cursor_x.saturating_sub(1),
            event::KeyCode::Right => self.cursor_x += 1,
            event::KeyCode::Up => self.cursor_y = self.cursor_y.saturating_sub(1),
            event::KeyCode::Down => self.cursor_y += 1,
            event::KeyCode::PageUp => self.cursor_y = 0,
            event::KeyCode::PageDown => self.cursor_y = self.screen_rows - 1,
            event::KeyCode::Home => self.cursor_x = 0,
            event::KeyCode::End => self.cursor_x = self.screen_cols - 1,
            _ => {}
        }

        if self.cursor_x + 1 > self.screen_cols {
            self.cursor_x = self.screen_cols - 1;
        }

        if self.cursor_y + 1 > self.screen_rows {
            self.cursor_y = self.screen_rows - 1;
        }

        ControlFlow::Continue
    }
}

pub enum ControlFlow {
    Continue,
    Break,
}
