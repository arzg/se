use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    terminal,
};

fn main() -> anyhow::Result<()> {
    terminal::enable_raw_mode()?;

    loop {
        if let Event::Key(KeyEvent { code, modifiers }) = event::read()? {
            if code == KeyCode::Char('q') {
                break;
            }
        }
    }

    terminal::disable_raw_mode()?;

    Ok(())
}
