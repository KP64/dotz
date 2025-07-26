//! A simple feature rich Colorscript.

use clap::Parser as _;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute, queue,
    style::{Color, Print, SetForegroundColor},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use dotz::{Cli, Mode};
use rand::seq::IndexedMutRandom as _;
use std::{
    error,
    io::{self, Write},
    time::Duration,
};

/// # Returns
/// the terminal area (x * y)
fn terminal_area_size() -> io::Result<usize> {
    let (cols, rows) = terminal::size()?;
    let area = usize::from(cols).saturating_mul(usize::from(rows));
    Ok(area)
}

/// Fills the terminal screen with `cli.char` in random colors,
/// then blocks until the user quits.
fn fill_screen<W>(mut writer: W, char: char) -> io::Result<()>
where
    W: Write,
{
    let area = terminal_area_size()?;
    for _ in 0..area {
        queue!(writer, SetForegroundColor(generate_color()), Print(char))?;
    }
    writer.flush()?;

    while !is_quitting_char_read(Duration::MAX)? {}
    Ok(())
}

/// Generate a random ANSI Xterm system color.
/// This is done to fully support Xterm and increase usability.
///
/// # Returns
///
/// A `Color::AnsiValue` in the range 0..16.
#[must_use]
fn generate_color() -> Color {
    Color::AnsiValue(rand::random_range(0..16))
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

/// Returns the input polling duration
fn get_duration(ips: f64) -> Result<Duration, String> {
    let key_wait_dur = 1.0 / ips;
    let Ok(dur) = Duration::try_from_secs_f64(key_wait_dur) else {
        return Err(format!("Cannot divide by {ips}"));
    };
    Ok(dur)
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let Cli { char, mode } = Cli::parse();

    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, cursor::Hide)?;

    match mode {
        Some(Mode::FillScreen) | None => fill_screen(&stdout, char),
        Some(Mode::Infinite { speed }) => {
            let dur = get_duration(speed.ips)?;
            print_infinite(&stdout, char, dur)
        }
        Some(Mode::Random { speed }) => {
            let dur = get_duration(speed.ips)?;
            print_random(&stdout, char, dur)
        }
        Some(Mode::Spaced {
            separator,
            spaces,
            speed,
        }) => {
            let dur = get_duration(speed.ips)?;
            print_spaced(&stdout, char, dur, separator, spaces)
        }
    }?;

    execute!(stdout, LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

/// Prints the Spaced mode
fn print_spaced<W>(
    mut writer: W,
    char: char,
    dur: Duration,
    separator: char,
    spaces: usize,
) -> io::Result<()>
where
    W: Write,
{
    // Do not print the separators first,
    // because it looks ugly xD
    let mut counter = spaces;

    while !is_quitting_char_read(dur)? {
        let ch = if spaces.saturating_sub(counter) == 0 {
            counter = 0;
            char
        } else {
            counter = counter.wrapping_add(1);
            separator
        };
        execute!(writer, SetForegroundColor(generate_color()), Print(ch))?;
    }
    Ok(())
}

/// Continuously print `cli.char` at the current cursor position in random colors.
fn print_infinite<W>(mut writer: W, char: char, dur: Duration) -> io::Result<()>
where
    W: Write,
{
    while !is_quitting_char_read(dur)? {
        execute!(writer, SetForegroundColor(generate_color()), Print(char))?;
    }
    Ok(())
}

/// Renders a grid of characters, changing the color of a single cell with each iteration.
fn print_random<W>(mut writer: W, char: char, dur: Duration) -> io::Result<()>
where
    W: Write,
{
    let area = terminal_area_size()?;
    let mut grid = vec![None; area];

    let mut rng = rand::rng();
    while !is_quitting_char_read(dur)? {
        let Some(ele) = grid.choose_mut(&mut rng) else {
            continue;
        };
        *ele = Some(generate_color());

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
