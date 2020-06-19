#![warn(rust_2018_idioms)]

use crossterm::{cursor, event, queue, terminal};
use std::convert::{TryFrom, TryInto};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::time::{Duration, SystemTime};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

const WELCOME_MSG: &str = concat!("se v", env!("CARGO_PKG_VERSION"), " · A screen editor.");
const STATUS_BAR_HEIGHT: usize = 1;
const STATUS_MSG_HEIGHT: usize = 1;
const STATUS_MSG_TIMEOUT: Duration = Duration::from_secs(5);

pub struct Editor {
    cursor_x: usize,
    cursor_y: usize,
    screen_rows: usize,
    screen_cols: usize,
    editor_rows: usize,
    editor_cols: usize,
    row_offset: usize,
    col_offset: usize,
    buffer: Vec<String>,
    path: Option<PathBuf>,
    status_msg: Option<StatusMsg>,
    renderer: Renderer,
}

impl Editor {
    pub fn open(path: Option<PathBuf>) -> anyhow::Result<Self> {
        let (screen_cols, screen_rows) = terminal::size()?;

        let screen_rows = usize::try_from(screen_rows)?;
        let screen_cols = usize::try_from(screen_cols)?;

        let (editor_cols, editor_rows) =
            convert_screen_dimens_to_editor_dimens(screen_cols, screen_rows);

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
            screen_rows,
            screen_cols,
            editor_rows,
            editor_cols,
            row_offset: 0,
            col_offset: 0,
            buffer,
            path,
            status_msg: None,
            renderer: Renderer::new(screen_rows),
        })
    }

    pub fn refresh_screen(
        &mut self,
        stdout: &mut io::Stdout,
        refresh: Refresh,
    ) -> anyhow::Result<()> {
        queue!(stdout, cursor::Hide, cursor::MoveTo(0, 0))?;

        let mut rendered = Vec::with_capacity(self.screen_cols * self.screen_rows);

        self.draw_rows(&mut rendered)?;
        self.draw_status_bar(&mut rendered)?;
        self.draw_status_msg(&mut rendered)?;

        self.renderer.update(rendered);
        self.renderer.render(stdout, refresh)?;

        let graphemes = self.buffer[self.cursor_y].graphemes(true);
        let graphemes_from_offset_to_cursor = graphemes.take(self.cursor_x).skip(self.col_offset);

        let screen_cursor_x: u16 = graphemes_from_offset_to_cursor
            .collect::<String>()
            .width()
            .try_into()?;
        let screen_cursor_y: u16 = (self.cursor_y - self.row_offset).try_into()?;

        queue!(
            stdout,
            cursor::MoveTo(screen_cursor_x, screen_cursor_y),
            cursor::Show
        )?;

        stdout.flush()?;

        Ok(())
    }

    fn draw_rows(&self, writer: &mut impl Write) -> anyhow::Result<()> {
        for i in 0..self.editor_rows {
            if let Some(line) = self.buffer.get(i + self.row_offset) {
                let graphemes = line.graphemes(true);
                let width = line.width();
                let reaches_left_of_editor = width > self.col_offset;

                let line: String = if reaches_left_of_editor {
                    slice_graphemes(graphemes.skip(self.col_offset), self.editor_cols)
                } else {
                    String::new()
                };

                write!(writer, "{}", line)?;
            } else {
                self.draw_empty_row(writer, i)?;
            }

            queue!(writer, terminal::Clear(terminal::ClearType::UntilNewLine))?;
            writeln!(writer)?;
        }

        Ok(())
    }

    fn draw_empty_row(&self, writer: &mut impl Write, i: usize) -> anyhow::Result<()> {
        write!(writer, "~")?;

        // Only draw welcome message if the buffer is empty.
        if self.buffer.is_empty() && i == self.editor_rows / 3 {
            let padding_len = (self.editor_cols - WELCOME_MSG.len()) / 2;
            let padding = " ".repeat(padding_len);

            write!(writer, "{}{}", padding, WELCOME_MSG)?;
        }

        Ok(())
    }

    fn draw_status_bar(&self, writer: &mut impl Write) -> anyhow::Result<()> {
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
        writeln!(writer, "{}", reverse.paint(status_bar))?;

        Ok(())
    }

    fn draw_status_msg(&self, writer: &mut impl Write) -> anyhow::Result<()> {
        let status_msg = if let Some(ref status_msg) = self.status_msg {
            if status_msg.timestamp.elapsed()? < STATUS_MSG_TIMEOUT {
                Some(status_msg)
            } else {
                None
            }
        } else {
            None
        };

        if let Some(status_msg) = status_msg {
            let graphemes = status_msg.text.graphemes(true);
            write!(writer, "{}", slice_graphemes(graphemes, self.screen_cols))?;
        } else {
            queue!(writer, terminal::Clear(terminal::ClearType::CurrentLine))?;
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
            event::KeyCode::PageUp => {
                self.cursor_y = self.cursor_y.saturating_sub(self.editor_rows / 2)
            }
            event::KeyCode::PageDown => self.cursor_y += self.editor_rows / 2,
            event::KeyCode::Home => self.cursor_x = 0,
            event::KeyCode::End => self.cursor_x = self.buffer[self.cursor_y].len(),
            event::KeyCode::Char(c) => self.insert_char(c),
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

        let num_graphemes = self.buffer[self.cursor_y].graphemes(true).count();
        if self.cursor_x + 1 > num_graphemes {
            self.cursor_x = num_graphemes;
        }
    }

    fn scroll(&mut self) {
        if self.cursor_y < self.row_offset {
            self.row_offset = self.cursor_y;
        }

        if self.cursor_y >= self.row_offset + self.editor_rows {
            self.row_offset = self.cursor_y - self.editor_rows + 1;
        }

        if self.cursor_x < self.col_offset {
            self.col_offset = self.cursor_x;
        }

        if self.cursor_x >= self.col_offset + self.editor_cols {
            self.col_offset = self.cursor_x - self.editor_cols + 1;
        }
    }

    pub fn insert_char(&mut self, c: char) {
        if self.cursor_y == self.buffer.len() {
            self.buffer.push(String::new());
        }

        let cursor_x_byte_pos = self.cursor_x_byte_pos();
        self.buffer[self.cursor_y].insert(cursor_x_byte_pos, c);
        self.cursor_x += 1; // A char can only be one grapheme
    }

    pub fn resize(&mut self, cols: usize, rows: usize) {
        self.screen_cols = cols;
        self.screen_rows = rows;

        let (editor_cols, editor_rows) =
            convert_screen_dimens_to_editor_dimens(self.screen_cols, self.screen_rows);
        self.editor_cols = editor_cols;
        self.editor_rows = editor_rows;

        // We need to re-limit the cursor position to the file and to the screen to prevent the
        // cursor from going offscreen.
        self.scroll();
    }

    pub fn set_status_msg(&mut self, msg: String) {
        self.status_msg = Some(StatusMsg {
            text: msg,
            timestamp: SystemTime::now(),
        });
    }

    fn cursor_x_byte_pos(&self) -> usize {
        let graphemes = self.buffer[self.cursor_y].graphemes(true);
        graphemes.take(self.cursor_x).collect::<String>().len()
    }
}

#[derive(PartialEq)]
pub enum Refresh {
    Full,
    Partial,
}

struct StatusMsg {
    text: String,
    timestamp: SystemTime,
}

struct Renderer {
    current: Vec<Vec<u8>>,
    previous: Vec<Vec<u8>>,
}

impl Renderer {
    fn new(lines: usize) -> Self {
        Self {
            current: vec![Vec::new(); lines],
            previous: vec![Vec::new(); lines],
        }
    }

    fn update(&mut self, new: Vec<u8>) {
        let new: Vec<_> = new.split(|b| is_whitespace(*b)).map(Vec::from).collect();

        debug_assert_eq!(self.current.len(), new.len());

        self.previous = self.current.clone();
        self.current = new;
    }

    fn render(&self, writer: &mut impl Write, refresh: Refresh) -> anyhow::Result<()> {
        queue!(writer, cursor::Hide, cursor::MoveTo(0, 0))?;

        for (i, (current_line, previous_line)) in
            self.current.iter().zip(&self.previous).enumerate()
        {
            if refresh == Refresh::Full || current_line != previous_line {
                let i = i.try_into()?;
                queue!(writer, cursor::MoveTo(0, i))?;

                writer.write_all(current_line)?;
            }
        }

        Ok(())
    }
}

pub enum ControlFlow {
    Continue,
    Break,
}

fn slice_graphemes<'a>(graphemes: impl Iterator<Item = &'a str>, max_width: usize) -> String {
    let mut output = String::with_capacity(max_width);

    // Keep adding graphemes to the output while they’re small enough to fit.
    for grapheme in graphemes {
        if output.width() + grapheme.width() <= max_width {
            output.push_str(grapheme);
        } else {
            break;
        }
    }

    output
}

fn convert_screen_dimens_to_editor_dimens(
    screen_cols: usize,
    screen_rows: usize,
) -> (usize, usize) {
    (
        screen_cols,
        screen_rows - STATUS_BAR_HEIGHT - STATUS_MSG_HEIGHT,
    )
}

fn is_whitespace(byte: u8) -> bool {
    byte == b'\n'
}
