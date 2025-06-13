//! A simple feature rich Colorscript.

use clap::{Parser, ValueEnum};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute, queue,
    style::{Color, Print, SetForegroundColor},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::seq::IndexedMutRandom as _;
use std::{
    error, fmt,
    io::{self, Write},
    time::Duration,
};

/// Command-line interface definition.
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The character to print
    #[arg(short, long, default_value_t = '.')]
    char: char,

    /// The loop iterations per second
    #[arg(short, long, default_value_t = 240.0)]
    ips: f64,

    /// The printing Mode
    #[arg(short, long, default_value_t = Mode::default())]
    mode: Mode,
}

/// Available printing modes.
#[derive(Debug, Default, Clone, ValueEnum)]
enum Mode {
    /// Fill the screen immediately
    FillScreen,

    /// Continuously print characters
    #[default]
    Infinite,

    /// Randomly color individual cells over time
    Random,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match *self {
            Self::FillScreen => "fill-screen",
            Self::Infinite => "infinite",
            Self::Random => "random",
        };
        f.write_str(str)
    }
}

/// Fills the terminal screen with `cli.char` in random colors,
/// then blocks until the user quits.
fn fill_screen<W>(char: char, mut writer: W) -> io::Result<()>
where
    W: Write,
{
    let (cols, rows) = terminal::size()?;
    for _ in 0..cols {
        for _ in 0..rows {
            queue!(writer, SetForegroundColor(generate_color()), Print(char))?;
        }
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

fn main() -> Result<(), Box<dyn error::Error>> {
    let Cli { char, mode, ips } = Cli::parse();

    let key_wait_dur = 1.0 / ips;
    let Ok(dur) = Duration::try_from_secs_f64(key_wait_dur) else {
        return Err(format!("Cannot divide by {ips}").into());
    };

    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, cursor::Hide)?;

    match mode {
        Mode::FillScreen => fill_screen(char, &stdout),
        Mode::Infinite => print_infinite(char, &stdout, dur),
        Mode::Random => print_random(char, &stdout, dur),
    }?;

    execute!(stdout, LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

/// Continuously print `cli.char` at the current cursor position in random colors.
fn print_infinite<W>(char: char, mut writer: W, dur: Duration) -> io::Result<()>
where
    W: Write,
{
    while !is_quitting_char_read(dur)? {
        execute!(writer, SetForegroundColor(generate_color()), Print(char))?;
    }
    Ok(())
}

/// Renders a grid of characters, changing the color of a single cell with each iteration.
fn print_random<W>(char: char, mut writer: W, dur: Duration) -> io::Result<()>
where
    W: Write,
{
    let (columns, rows) = terminal::size()?;
    let mut grid = vec![vec![None; usize::from(columns)]; usize::from(rows)];

    let mut rng = rand::rng();
    while !is_quitting_char_read(dur)? {
        let Some(vec) = grid.choose_mut(&mut rng) else {
            continue;
        };
        let Some(ele) = vec.choose_mut(&mut rng) else {
            continue;
        };
        *ele = Some(generate_color());

        for row in &grid {
            for cell in row {
                if let Some(color) = *cell {
                    queue!(writer, SetForegroundColor(color), Print(char))?;
                } else {
                    queue!(writer, Print(' '))?;
                }
            }
        }
        writer.flush()?;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::Cli;
    use clap::CommandFactory as _;

    /// Verifies that the CLI flags/options do not conflict.
    #[test]
    fn verify_cli() {
        Cli::command().debug_assert();
    }
}
