use crossterm::{
    event::{self, Event},
    terminal,
};

fn main() -> anyhow::Result<()> {
    terminal::enable_raw_mode()?;

    loop {
        if let Event::Key(key_event) = event::read()? {
            if let se::ControlFlow::Break = se::process_keypress(key_event) {
                break;
            }
        }
    }

    terminal::disable_raw_mode()?;

    Ok(())
}
