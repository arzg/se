#![warn(rust_2018_idioms)]

use crossterm::{cursor, event, queue, terminal};
use std::convert::TryInto;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

const WELCOME_MSG: &str = concat!("se v", env!("CARGO_PKG_VERSION"), " · A screen editor.");

pub struct Editor {
    cursor_x: usize,
    cursor_y: usize,
    screen_rows: usize,
    screen_cols: usize,
    row_offset: usize,
    col_offset: usize,
    buffer: Vec<String>,
}

impl Editor {
    pub fn open(path: Option<impl AsRef<Path>>) -> anyhow::Result<Self> {
        let (cols, rows) = terminal::size()?;

        let buffer = if let Some(path) = path {
            fs::read_to_string(path)?
                .lines()
                .map(String::from)
                .collect()
        } else {
            Vec::new()
        };

        Ok(Self {
            cursor_x: 0,
            cursor_y: 0,
            screen_rows: rows.try_into()?,
            screen_cols: cols.try_into()?,
            row_offset: 0,
            col_offset: 0,
            buffer,
        })
    }

    pub fn refresh_screen(&self, stdout: &mut io::Stdout) -> anyhow::Result<()> {
        queue!(stdout, cursor::Hide, cursor::MoveTo(0, 0))?;

        self.draw_rows(stdout)?;

        let screen_cursor_x: u16 = (self.cursor_x - self.col_offset).try_into()?;
        let screen_cursor_y: u16 = (self.cursor_y - self.row_offset).try_into()?;
        queue!(
            stdout,
            cursor::MoveTo(screen_cursor_x, screen_cursor_y),
            cursor::Show
        )?;

        stdout.flush()?;

        Ok(())
    }

    fn draw_rows(&self, stdout: &mut io::Stdout) -> anyhow::Result<()> {
        let is_on_last_row = |i| i == self.screen_rows - 1;

        for i in 0..self.screen_rows {
            if let Some(line) = self.buffer.get(i + self.row_offset) {
                let line_len = line.len();
                let reaches_to_left_of_screen = line_len > self.col_offset;
                let reaches_to_right_of_screen = line_len >= self.col_offset + self.screen_cols;

                let line = match (reaches_to_left_of_screen, reaches_to_right_of_screen) {
                    // If a line reaches to the right of the screen it must also reach the left of
                    // the screen.
                    (_, true) => &line[self.col_offset..self.col_offset + self.screen_cols],
                    (true, _) => &line[self.col_offset..],
                    _ => "",
                };

                write!(stdout, "{}", line)?;
            } else {
                self.draw_empty_row(stdout, i)?;
            }

            queue!(stdout, terminal::Clear(terminal::ClearType::UntilNewLine))?;

            if !is_on_last_row(i) {
                writeln!(stdout, "\r")?;
            }
        }

        Ok(())
    }

    fn draw_empty_row(&self, stdout: &mut io::Stdout, i: usize) -> anyhow::Result<()> {
        write!(stdout, "~")?;

        // Only draw welcome message if the buffer is empty.
        if self.buffer.is_empty() && i == self.screen_rows / 3 {
            let padding_len = (self.screen_cols - WELCOME_MSG.len()) / 2;
            let padding = " ".repeat(padding_len);

            write!(stdout, "{}{}", padding, WELCOME_MSG)?;
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

        let num_lines = self.buffer.len();
        if self.cursor_y + 1 > num_lines {
            self.cursor_y = num_lines.saturating_sub(1);
        }

        let num_chars = self.buffer.get(self.cursor_y).map(String::len).unwrap_or(0);
        if self.cursor_x + 1 > num_chars {
            self.cursor_x = num_chars.saturating_sub(1);
        }

        self.scroll();

        ControlFlow::Continue
    }

    fn scroll(&mut self) {
        if self.cursor_y < self.row_offset {
            self.row_offset = self.cursor_y;
        }

        if self.cursor_y >= self.row_offset + self.screen_rows {
            self.row_offset = self.cursor_y - self.screen_rows + 1;
        }

        if self.cursor_x < self.col_offset {
            self.col_offset = self.cursor_x;
        }

        if self.cursor_x >= self.col_offset + self.screen_cols {
            self.col_offset = self.cursor_x - self.screen_cols + 1;
        }
    }
}

pub enum ControlFlow {
    Continue,
    Break,
}
