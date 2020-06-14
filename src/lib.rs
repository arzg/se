#![warn(rust_2018_idioms)]

use crossterm::{cursor, event, queue, terminal};
use std::convert::TryInto;
use std::io::{self, Write};

const WELCOME_MSG: &str = concat!("se v", env!("CARGO_PKG_VERSION"), " · A screen editor.");

pub struct Editor {
    cursor_x: usize,
    cursor_y: usize,
    screen_rows: usize,
    screen_cols: usize,
    line: String,
}

impl Editor {
    pub fn new() -> anyhow::Result<Self> {
        let (cols, rows) = terminal::size()?;

        Ok(Self {
            cursor_x: 0,
            cursor_y: 0,
            screen_rows: rows.try_into()?,
            screen_cols: cols.try_into()?,
            line: "Hello, world!".to_string(),
        })
    }

    pub fn refresh_screen(&self, stdout: &mut io::Stdout) -> anyhow::Result<()> {
        queue!(stdout, cursor::Hide, cursor::MoveTo(0, 0))?;

        self.draw_rows(stdout)?;

        let cursor_x: u16 = self.cursor_x.try_into()?;
        let cursor_y: u16 = self.cursor_y.try_into()?;
        queue!(stdout, cursor::MoveTo(cursor_x, cursor_y), cursor::Show)?;

        stdout.flush()?;

        Ok(())
    }

    fn draw_rows(&self, stdout: &mut io::Stdout) -> anyhow::Result<()> {
        for i in 0..self.screen_rows {
            if i == 0 {
                let line = if self.line.len() > self.screen_cols {
                    &self.line[0..self.screen_cols + 1]
                } else {
                    &self.line
                };

                writeln!(stdout, "{}\r", line)?;
            } else {
                self.draw_empty_row(stdout, i)?;
            }
        }

        Ok(())
    }

    fn draw_empty_row(&self, stdout: &mut io::Stdout, i: usize) -> anyhow::Result<()> {
        let is_on_last_row = |i| i == self.screen_rows - 1;

        write!(stdout, "~")?;

        if i == self.screen_rows / 3 {
            let padding_len = (self.screen_cols - WELCOME_MSG.len()) / 2;
            let padding = " ".repeat(padding_len);

            write!(stdout, "{}{}", padding, WELCOME_MSG)?;
        }

        queue!(stdout, terminal::Clear(terminal::ClearType::UntilNewLine))?;

        if !is_on_last_row(i) {
            writeln!(stdout, "\r")?;
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
