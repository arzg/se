#![warn(rust_2018_idioms)]

mod width_limited;

use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers};
use crossterm::{cursor, queue, terminal};
use std::convert::{TryFrom, TryInto};
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::time::{Duration, SystemTime};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;
use width_limited::WidthLimited;

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
    highlighted_buffer: Vec<Vec<dialect::StyledGrapheme>>,
    path: Option<PathBuf>,
    status_msg: Option<StatusMsg>,
    renderer: Renderer,
    unmodified_hash: u64,
    is_modified: bool,
    quit_without_saving: bool,
}

impl Editor {
    const HIGHLIGHTER: syntax_rust::Highlighter = syntax_rust::Highlighter;
    const THEME: dialect::themes::DarkPlus = dialect::themes::DarkPlus;

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
            vec![String::new()]
        };

        let initial_hash = compute_hash(&buffer);

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
            highlighted_buffer: Vec::new(),
            path,
            status_msg: None,
            renderer: Renderer::new(screen_rows),
            unmodified_hash: initial_hash,
            is_modified: false,
            quit_without_saving: false,
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
            if let Some(line) = self.highlighted_buffer.get(i + self.row_offset) {
                let scrolled =
                    WidthLimited::new(line.iter().skip(self.col_offset), self.editor_cols);

                let rendered = scrolled.map(|styled_grapheme| {
                    let style = ansi_term::Style::from(styled_grapheme.style);
                    style.paint(styled_grapheme.grapheme.as_str()).to_string()
                });

                write!(writer, "{}", rendered.collect::<String>())?;
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
        let modified_flag = if self.is_modified { "[modified]" } else { "" };
        let left_status_bar = format!("{} - {} {}", filename, line_count, modified_flag);

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
            write!(
                writer,
                "{}",
                WidthLimited::new(graphemes, self.screen_cols).collect::<String>()
            )?;
        } else {
            queue!(writer, terminal::Clear(terminal::ClearType::CurrentLine))?;
        }

        Ok(())
    }

    pub fn process_keypress(&mut self, key_event: event::KeyEvent) -> ControlFlow {
        // The KeyEvents provided by crossterm do not contain capital letters -- instead, they
        // contain a lowercase char and a shift modifier. Here, this is converted to an uppercase
        // char with no modifiers.
        let key_event = match key_event {
            KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::SHIFT,
            } => {
                // Converting a char to uppercase can result in more than one char. To allow for
                // this, we process each of the characters (apart from the last one) through the use
                // of recursion. Then, we finally process the last character non-recursively.

                let uppercase_chars: Vec<_> = c.to_uppercase().collect();

                for c in uppercase_chars[..uppercase_chars.len() - 1].iter() {
                    self.process_keypress(KeyEvent {
                        code: KeyCode::Char(*c),
                        modifiers: KeyModifiers::NONE,
                    });
                }

                KeyEvent {
                    // We can unwrap because converting a character to uppercase always results in
                    // at least one character, so the last element of the vector must exist.
                    code: KeyCode::Char(*uppercase_chars.last().unwrap()),
                    modifiers: KeyModifiers::NONE,
                }
            }
            _ => key_event,
        };

        match key_event {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL,
            } => match (self.is_modified, self.quit_without_saving) {
                (true, true) | (false, _) => return ControlFlow::Break,
                (true, false) => {
                    self.set_status_msg(
                        "Buffer is modified! Press C-q again to quit without saving.".to_string(),
                    );
                    self.quit_without_saving = true;
                    return ControlFlow::Continue;
                }
            },

            KeyEvent {
                code: KeyCode::Char('s'),
                modifiers: KeyModifiers::CONTROL,
            } => self.save(),

            KeyEvent {
                code,
                modifiers: KeyModifiers::NONE,
            } => match code {
                KeyCode::Left => self.cursor_x = self.cursor_x.saturating_sub(1),
                KeyCode::Right => self.cursor_x += 1,
                KeyCode::Up => self.cursor_y = self.cursor_y.saturating_sub(1),
                KeyCode::Down => self.cursor_y += 1,
                KeyCode::Backspace => self.backspace(),
                KeyCode::Enter => self.enter(),
                KeyCode::PageUp => {
                    self.cursor_y = self.cursor_y.saturating_sub(self.editor_rows / 2)
                }
                KeyCode::PageDown => self.cursor_y += self.editor_rows / 2,
                KeyCode::Home => self.cursor_x = 0,
                KeyCode::End => self.cursor_x = self.buffer[self.cursor_y].graphemes(true).count(),
                KeyCode::Char(c) => self.insert_char(c),
                _ => {}
            },

            _ => {}
        }

        // If quit_without_saving is true and the user presses C-q, then it is handled above.
        // Otherwise, it is not needed anymore and can be set back to false.
        self.quit_without_saving = false;

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

    fn insert_char(&mut self, c: char) {
        if self.cursor_y == self.buffer.len() {
            self.buffer.push(String::new());
        }

        let cursor_x_byte_pos = self.cursor_x_byte_pos();
        self.buffer[self.cursor_y].insert(cursor_x_byte_pos, c);
        self.cursor_x += 1; // A char can only be one grapheme

        self.update_on_modified_buffer();
    }

    fn backspace(&mut self) {
        if self.cursor_x == 0 {
            if self.cursor_y == 0 {
                return;
            }

            let above_row_num_graphemes = self.buffer[self.cursor_y - 1].graphemes(true).count();

            let current_row = self.buffer[self.cursor_y].clone();
            self.buffer[self.cursor_y - 1].push_str(&current_row);

            self.buffer.remove(self.cursor_y);

            self.cursor_y -= 1;
            self.cursor_x = above_row_num_graphemes;
        } else {
            self.buffer[self.cursor_y] = self.buffer[self.cursor_y]
                .graphemes(true)
                .enumerate()
                // Remove the grapheme whose index is one less than the cursor’s position; in other
                // words, remove the grapheme before the cursor.
                .filter_map(|(idx, grapheme)| {
                    if idx == self.cursor_x - 1 {
                        None
                    } else {
                        Some(grapheme)
                    }
                })
                .collect();

            self.cursor_x -= 1;
        }

        self.update_on_modified_buffer();
    }

    fn enter(&mut self) {
        let graphemes = self.buffer[self.cursor_y].graphemes(true);
        let line_before_cursor = graphemes.clone().take(self.cursor_x).collect();
        let line_after_cursor = graphemes.skip(self.cursor_x).collect();

        self.buffer[self.cursor_y] = line_before_cursor;
        self.buffer.insert(self.cursor_y + 1, line_after_cursor);

        self.cursor_y += 1;
        self.cursor_x = 0;

        self.update_on_modified_buffer();
    }

    fn update_on_modified_buffer(&mut self) {
        self.update_highlighting();
        self.update_modified_status();
    }

    fn update_highlighting(&mut self) {
        let buffer = self.buffer.join("\n");
        let styled_graphemes = dialect::render(&buffer, Self::HIGHLIGHTER, Self::THEME);

        // Split by lines.
        self.highlighted_buffer = styled_graphemes
            .split(|styled_grapheme| styled_grapheme.grapheme.contains('\n'))
            .map(Vec::from)
            .collect();
    }

    fn update_modified_status(&mut self) {
        let current_hash = compute_hash(&self.buffer);
        self.is_modified = self.unmodified_hash != current_hash;
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

    fn save(&mut self) {
        if let Some(ref path) = self.path {
            let save = || -> anyhow::Result<usize> {
                let file_contents = {
                    let mut s = self.buffer.join("\n");
                    s.push_str("\n"); // Always add trailing newline.
                    s
                };

                fs::write(path, &file_contents)?;

                Ok(file_contents.len())
            };

            match save() {
                Ok(num_bytes) => {
                    let status_msg = format!("Wrote {} bytes to {}", num_bytes, path.display());
                    self.set_status_msg(status_msg);

                    // Update the hash modified comparisons are based upon.
                    self.unmodified_hash = compute_hash(&self.buffer);
                    self.is_modified = false;
                }
                Err(e) => {
                    let status_msg = format!("Error while saving {}: {}", path.display(), e);
                    self.set_status_msg(status_msg)
                }
            };
        } else {
            self.set_status_msg("This buffer does not have a path to save at".to_string());
        }
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

        self.previous = self.current.clone();
        self.current = new;
    }

    fn render(&self, writer: &mut impl Write, refresh: Refresh) -> anyhow::Result<()> {
        queue!(writer, cursor::Hide, cursor::MoveTo(0, 0))?;

        let mut write_line = |i: usize, line| -> anyhow::Result<()> {
            let i = i.try_into()?;
            queue!(writer, cursor::MoveTo(0, i))?;

            writer.write_all(line)?;

            Ok(())
        };

        let mut i = 0;

        loop {
            match (self.current.get(i), self.previous.get(i)) {
                (Some(current_line), Some(previous_line)) => {
                    if refresh == Refresh::Full || current_line != previous_line {
                        write_line(i, current_line)?;
                    }
                }
                (Some(current_line), None) => write_line(i, current_line)?,
                (None, Some(_)) => {}
                (None, None) => break,
            }

            i += 1;
        }

        Ok(())
    }
}

pub enum ControlFlow {
    Continue,
    Break,
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

fn compute_hash(buffer: &[String]) -> u64 {
    seahash::hash(buffer.join("\n").as_bytes())
}

fn is_whitespace(byte: u8) -> bool {
    byte == b'\n'
}
