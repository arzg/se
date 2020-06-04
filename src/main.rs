use {std::path::PathBuf, structopt::StructOpt};

#[derive(StructOpt)]
struct Opts {
    path: PathBuf,
}

fn main() -> anyhow::Result<()> {
    use {
        crossterm::{
            event::{self, KeyCode, KeyModifiers},
            queue, terminal,
        },
        std::io::Write,
    };

    let opts = Opts::from_args();

    // Attempt to load the given file before doing anything else.
    let mut buffer = se::Buffer::new(opts.path)?;

    let mut stdout = std::io::stdout();

    queue!(stdout, terminal::EnterAlternateScreen)?;
    terminal::enable_raw_mode()?;

    buffer.initialize_terminal(&mut stdout)?;
    buffer.redraw(&mut stdout)?;

    loop {
        if let event::Event::Key(k) = event::read()? {
            match (k.code, k.modifiers) {
                (c, KeyModifiers::NONE) => match c {
                    KeyCode::Up => buffer.move_cursor(se::Direction::Up),
                    KeyCode::Down => buffer.move_cursor(se::Direction::Down),
                    KeyCode::Left => buffer.move_cursor(se::Direction::Left),
                    KeyCode::Right => buffer.move_cursor(se::Direction::Right),
                    KeyCode::Backspace => buffer.backspace(),
                    KeyCode::Enter => buffer.insert_newline(),
                    KeyCode::Char(c) => buffer.insert_char(c),
                    _ => (),
                },
                // Quit on C-q
                (KeyCode::Char('q'), KeyModifiers::CONTROL) => break,
                _ => (),
            }
        }

        buffer.redraw(&mut stdout)?;
    }

    terminal::disable_raw_mode()?;
    queue!(stdout, terminal::LeaveAlternateScreen)?;
    stdout.flush()?;

    Ok(())
}
