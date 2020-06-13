use crossterm::event::{self, Event};
use crossterm::{cursor, execute, terminal};
use std::io::{self, Write};

fn main() -> anyhow::Result<()> {
    terminal::enable_raw_mode()?;

    let mut stdout = io::stdout();

    if let Err(e) = run(&mut stdout) {
        execute!(
            stdout,
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0)
        )?;

        eprintln!("{:?}", e);
    }

    terminal::disable_raw_mode()?;

    Ok(())
}

fn run(stdout: &mut io::Stdout) -> anyhow::Result<()> {
    se::refresh_screen(stdout)?;

    loop {
        if let Event::Key(key_event) = event::read()? {
            se::refresh_screen(stdout)?;

            if let se::ControlFlow::Break = se::process_keypress(key_event) {
                break;
            }
        }
    }

    Ok(())
}
