//! A simple feature rich Colorscript.

use clap::Parser as _;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute, queue,
    style::{Print, SetForegroundColor},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use dotz::{Cli, Mode};
use rand::seq::IndexedMutRandom as _;
use std::{error, io, time::Duration};

/// Fills the terminal screen with `cli.char` in random colors,
/// then blocks until the user quits.
fn fill_screen<W>(mut writer: W, char: char) -> io::Result<()>
where
    W: io::Write,
{
    let area = dotz::terminal_area_size()?;
    for _ in 0..area {
        queue!(
            writer,
            SetForegroundColor(dotz::generate_ansi_color()),
            Print(char)
        )?;
    }
    writer.flush()?;

    while !is_quitting_char_read(Duration::MAX)? {}
    Ok(())
}

/// Waits the given duration for a keypress and returns a bool
/// whether the Key quits the program.
fn is_quitting_char_read(dur: Duration) -> io::Result<bool> {
    if !event::poll(dur)? {
        return Ok(false);
    }
    Ok(matches!(
        event::read()?,
        Event::Key(
            KeyEvent {
                code: KeyCode::Char('q'),
                kind: KeyEventKind::Press,
                modifiers: KeyModifiers::NONE,
                ..
            } | KeyEvent {
                code: KeyCode::Char('c'),
                kind: KeyEventKind::Press,
                modifiers: KeyModifiers::CONTROL,
                ..
            },
        )
    ))
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let cli = Cli::parse();

    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, cursor::Hide)?;

    let mode = cli.mode.unwrap_or_default();
    match mode {
        Mode::FillScreen => fill_screen(&stdout, cli.char),
        Mode::Infinite { speed } => {
            let dur = dotz::get_duration(speed.ips)?;
            print_infinite(&stdout, cli.char, dur)
        }
        Mode::Random { speed } => {
            let dur = dotz::get_duration(speed.ips)?;
            print_random(&stdout, cli.char, dur)
        }
        Mode::Spaced {
            separator,
            spaces,
            speed,
        } => {
            let dur = dotz::get_duration(speed.ips)?;
            print_spaced(&stdout, cli.char, dur, separator, spaces)
        }
    }?;

    execute!(stdout, LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

/// Print the separator every few characters/spaces
fn print_spaced<W>(
    mut writer: W,
    char: char,
    dur: Duration,
    separator: char,
    spaces: u16,
) -> io::Result<()>
where
    W: io::Write,
{
    let mut chars_to_print = spaces;

    while !is_quitting_char_read(dur)? {
        let ch = if chars_to_print == 0 { separator } else { char };
        execute!(
            writer,
            SetForegroundColor(dotz::generate_ansi_color()),
            Print(ch)
        )?;
        chars_to_print = chars_to_print.checked_sub(1).unwrap_or(spaces);
    }
    Ok(())
}

/// Continuously print `cli.char` at the current cursor position in random colors.
fn print_infinite<W>(mut writer: W, char: char, dur: Duration) -> io::Result<()>
where
    W: io::Write,
{
    while !is_quitting_char_read(dur)? {
        execute!(
            writer,
            SetForegroundColor(dotz::generate_ansi_color()),
            Print(char)
        )?;
    }
    Ok(())
}

/// Renders a grid of characters, changing the color of a single cell with each iteration.
fn print_random<W>(mut writer: W, char: char, dur: Duration) -> io::Result<()>
where
    W: io::Write,
{
    let area = dotz::terminal_area_size()?;
    let mut grid = vec![None; area];

    let mut rng = rand::rng();
    while !is_quitting_char_read(dur)? {
        let _old_cell_color = grid
            .choose_mut(&mut rng)
            .map(|color| color.replace(dotz::generate_ansi_color()));

        for cell in &grid {
            if let Some(color) = *cell {
                queue!(writer, SetForegroundColor(color), Print(char))?;
            } else {
                queue!(writer, Print(' '))?;
            }
        }
        writer.flush()?;
    }
    Ok(())
}
