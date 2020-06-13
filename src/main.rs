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
    let mut editor = se::Editor::new()?;

    editor.refresh_screen(stdout)?;

    loop {
        if let Event::Key(key_event) = event::read()? {
            if let se::ControlFlow::Break = editor.process_keypress(key_event) {
                break;
            }

            editor.refresh_screen(stdout)?;
        }
    }

    Ok(())
}
