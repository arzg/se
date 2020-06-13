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

        if let event::KeyCode::Char(c) = code {
            match c {
                'a' => self.cursor_x -= 1,
                'd' => self.cursor_x += 1,
                'w' => self.cursor_y -= 1,
                's' => self.cursor_y += 1,
                _ => {}
            }
        }

        ControlFlow::Continue
    }
}

pub enum ControlFlow {
    Continue,
    Break,
}
