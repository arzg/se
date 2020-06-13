use crossterm::event::{self, Event, KeyCode, KeyEvent};

fn main() -> anyhow::Result<()> {
    loop {
        if let Event::Key(KeyEvent { code, modifiers }) = event::read()? {
            if code == KeyCode::Char('q') {
                break;
            }
        }
    }

    Ok(())
}
