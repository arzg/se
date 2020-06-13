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

            if let KeyCode::Char(c) = code {
                let mut buf = [0; 4];
                c.encode_utf8(&mut buf);

                println!("{:?} ('{}')\r", buf, c);
            }
        }
    }

    terminal::disable_raw_mode()?;

    Ok(())
}
