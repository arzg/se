use crossterm::event::{self, Event, KeyEvent};

fn main() -> anyhow::Result<()> {
    if let Event::Key(KeyEvent { code, modifiers }) = event::read()? {}

    Ok(())
}
