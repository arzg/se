use crossterm::event::{self, Event};
use crossterm::{cursor, execute, queue, terminal};
use std::convert::TryInto;
use std::io::{self, Write};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opts {
    path: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::from_args();

    terminal::enable_raw_mode()?;

    let mut stdout = io::stdout();

    let run_and_clear = {
        let result = run(opts, &mut stdout);

        execute!(
            stdout,
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0)
        )?;

        result
    };

    if let Err(e) = run_and_clear {
        eprintln!("{:?}", e);
    }

    terminal::disable_raw_mode()?;

    Ok(())
}

fn run(opts: Opts, stdout: &mut io::Stdout) -> anyhow::Result<()> {
    let mut editor = se::Editor::open(opts.path)?;

    queue!(stdout, terminal::Clear(terminal::ClearType::All))?;

    editor.refresh_screen(stdout)?;

    loop {
        match event::read()? {
            Event::Key(key_event) => {
                if let se::ControlFlow::Break = editor.process_keypress(key_event) {
                    break;
                }
            }
            Event::Resize(cols, rows) => editor.resize(cols.try_into()?, rows.try_into()?),
            _ => {}
        }

        editor.refresh_screen(stdout)?;
    }

    Ok(())
}
