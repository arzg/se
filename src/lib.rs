#![warn(rust_2018_idioms)]

use crossterm::{cursor, event, queue, terminal};
use std::convert::{TryFrom, TryInto};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

const WELCOME_MSG: &str = concat!("se v", env!("CARGO_PKG_VERSION"), " · A screen editor.");
const STATUS_BAR_HEIGHT: usize = 1;

pub struct Editor {
    cursor_x: usize,
    cursor_y: usize,
    screen_rows: usize,
    screen_cols: usize,
    row_offset: usize,
    col_offset: usize,
    buffer: Vec<String>,
    path: Option<PathBuf>,
}

impl Editor {
    pub fn open(path: Option<PathBuf>) -> anyhow::Result<Self> {
        let (cols, rows) = terminal::size()?;

        let buffer = if let Some(ref path) = path {
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
            screen_rows: usize::try_from(rows)? - STATUS_BAR_HEIGHT,
            screen_cols: usize::try_from(cols)?,
            row_offset: 0,
            col_offset: 0,
            buffer,
            path,
        })
    }

    pub fn refresh_screen(&self, stdout: &mut io::Stdout) -> anyhow::Result<()> {
        queue!(stdout, cursor::Hide, cursor::MoveTo(0, 0))?;

        self.draw_rows(stdout)?;
        self.draw_status_bar(stdout)?;

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
        for i in 0..self.screen_rows {
            if let Some(line) = self.buffer.get(i + self.row_offset) {
                let graphemes = line.graphemes(true);
                let width = line.width();
                let reaches_to_left_of_screen = width > self.col_offset;
                let reaches_to_right_of_screen = width >= self.col_offset + self.screen_cols;

                let line: String = match (reaches_to_left_of_screen, reaches_to_right_of_screen) {
                    // If a line reaches to the right of the screen it must also reach the left of
                    // the screen.
                    (_, true) => {
                        let mut line = String::with_capacity(self.screen_cols);

                        // Keep adding graphemes to the line while they’re small enough to fit.
                        for grapheme in graphemes.skip(self.col_offset) {
                            if line.width() + grapheme.width() <= self.screen_cols {
                                line.push_str(grapheme);
                            } else {
                                break;
                            }
                        }

                        line
                    }
                    (true, _) => graphemes.skip(self.col_offset).collect(),
                    _ => String::new(),
                };

                write!(stdout, "{}", line)?;
            } else {
                self.draw_empty_row(stdout, i)?;
            }

            queue!(stdout, terminal::Clear(terminal::ClearType::UntilNewLine))?;
            writeln!(stdout, "\r")?;
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

    fn draw_status_bar(&self, stdout: &mut io::Stdout) -> anyhow::Result<()> {
        use ansi_term::Style;
        use std::borrow::Cow;

        let filename = if let Some(ref path) = self.path {
            path.to_string_lossy()
        } else {
            Cow::Borrowed("[No Name]")
        };
        let line_count = format!("{} lines", self.buffer.len());
        let left_status_bar = format!("{} - {}", filename, line_count);

        let right_status_bar = format!("{}/{}", self.cursor_y + 1, self.buffer.len());

        // Require at least one character of padding between the left and right sides of the status
        // bar.
        const MIN_PADDING: usize = 1;

        let min_width_with_right = left_status_bar.width() + right_status_bar.width();
        let min_width_without_right = left_status_bar.width();

        let status_bar = if self.screen_cols >= min_width_with_right + MIN_PADDING {
            format!(
                "{}{}{}",
                left_status_bar,
                " ".repeat(self.screen_cols - min_width_with_right),
                right_status_bar
            )
        } else if self.screen_cols >= min_width_without_right {
            format!(
                "{}{}",
                left_status_bar,
                " ".repeat(self.screen_cols - min_width_without_right),
            )
        } else {
            " ".repeat(self.screen_cols)
        };

        let reverse = Style::new().reverse();
        write!(stdout, "{}", reverse.paint(status_bar))?;

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
            event::KeyCode::PageUp => {
                self.cursor_y = self.cursor_y.saturating_sub(self.screen_rows / 2)
            }
            event::KeyCode::PageDown => self.cursor_y += self.screen_rows / 2,
            event::KeyCode::Home => self.cursor_x = 0,
            event::KeyCode::End => self.cursor_x = self.buffer[self.cursor_y].len(),
            _ => {}
        }

        self.limit_cursor_pos_to_buffer_contents();
        self.scroll();

        ControlFlow::Continue
    }

    fn limit_cursor_pos_to_buffer_contents(&mut self) {
        let num_lines = self.buffer.len();
        if self.cursor_y + 1 > num_lines {
            self.cursor_y = num_lines.saturating_sub(1);
        }

        let width = self
            .buffer
            .get(self.cursor_y)
            .map(|line| line.width())
            .unwrap_or(0);

        if self.cursor_x + 1 > width {
            self.cursor_x = width.saturating_sub(1);
        }
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

    pub fn resize(&mut self, cols: usize, rows: usize) {
        self.screen_cols = cols;
        self.screen_rows = rows - STATUS_BAR_HEIGHT;

        // We need to re-limit the cursor position to the file and to the screen to prevent the
        // cursor from going offscreen.
        self.scroll();
    }
}

pub enum ControlFlow {
    Continue,
    Break,
}
