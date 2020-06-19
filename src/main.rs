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
    editor.set_status_msg("HELP: Ctrl-Q = quit".to_string());

    queue!(stdout, terminal::Clear(terminal::ClearType::All))?;

    editor.refresh_screen(stdout, se::Refresh::Full)?;

    loop {
        let refresh = match event::read()? {
            Event::Key(key_event) => {
                if let se::ControlFlow::Break = editor.process_keypress(key_event)? {
                    break;
                }
                Some(se::Refresh::Partial)
            }
            // Fully refresh the screen if the window was resized.
            Event::Resize(cols, rows) => {
                editor.resize(cols.try_into()?, rows.try_into()?);
                Some(se::Refresh::Full)
            }
            _ => None,
        };

        if let Some(refresh) = refresh {
            editor.refresh_screen(stdout, refresh)?;
        }
    }

    Ok(())
}
