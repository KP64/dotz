//! A simple feature rich Colorscript.

#![expect(
    clippy::missing_errors_doc,
    reason = "errors are mostly due to external factors"
)]

use clap::Parser as _;
use core::time::Duration;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute, queue,
    style::{Print, SetForegroundColor},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use dotz::{Cli, Mode};
use rand::RngExt as _;
use std::{error, io};

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
    let mut stdout = io::stdout().lock();
    execute!(stdout, EnterAlternateScreen, cursor::Hide)?;

    let mode = cli.mode.unwrap_or_default();
    match mode {
        Mode::FillScreen => fill_screen(&mut stdout, cli.char),
        Mode::Infinite { speed } => {
            let dur = dotz::get_duration(speed.ips)?;
            print_infinite(&mut stdout, cli.char, dur)
        }
        Mode::Random { speed } => {
            let dur = dotz::get_duration(speed.ips)?;
            print_random(&mut stdout, cli.char, dur)
        }
        Mode::Spaced {
            separator,
            spaces,
            speed,
        } => {
            let dur = dotz::get_duration(speed.ips)?;
            print_spaced(&mut stdout, cli.char, dur, separator, spaces)
        }
    }?;

    execute!(stdout, LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

/// Print the separator every few characters/spaces.
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
    (0..=spaces)
        .rev()
        .cycle()
        .map_while(|chars_to_print| {
            let ch = if chars_to_print == 0 { separator } else { char };
            is_quitting_char_read(dur)
                .is_ok_and(|should_quit| !should_quit)
                .then_some(ch)
        })
        .try_for_each(|ch| {
            execute!(
                writer,
                SetForegroundColor(dotz::generate_ansi_color()),
                Print(ch)
            )
        })
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
fn print_random<W>(mut writer: W, ch: char, dur: Duration) -> io::Result<()>
where
    W: io::Write,
{
    let (width, height) = terminal::size()?;

    let mut rng = rand::rng();

    while !is_quitting_char_read(dur)? {
        let x = rng.random_range(..width);
        let y = rng.random_range(..height);

        execute!(
            writer,
            cursor::MoveTo(x, y),
            SetForegroundColor(dotz::generate_ansi_color()),
            Print(ch)
        )?;
    }
    Ok(())
}
