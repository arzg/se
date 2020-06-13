use crossterm::event::{self, Event};
use crossterm::terminal;
use std::io;

fn main() -> anyhow::Result<()> {
    terminal::enable_raw_mode()?;

    let mut stdout = io::stdout();

    se::refresh_screen(&mut stdout)?;

    loop {
        if let Event::Key(key_event) = event::read()? {
            se::refresh_screen(&mut stdout)?;

            if let se::ControlFlow::Break = se::process_keypress(key_event) {
                break;
            }
        }
    }

    terminal::disable_raw_mode()?;

    Ok(())
}
