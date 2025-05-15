//! A simple feature rich Colorscript.

use clap::{Parser, ValueEnum};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute, queue,
    style::{Color, Print, SetForegroundColor},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    fmt,
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
        write!(f, "{str}")
    }
}

/// Fills the terminal screen with `cli.char` in random colors,
/// then blocks until the user quits.
fn fill_screen<W>(cli: &Cli, mut writer: W) -> io::Result<()>
where
    W: Write,
{
    let (cols, rows) = terminal::size()?;

    for _ in 0..cols {
        for _ in 0..rows {
            queue!(
                writer,
                SetForegroundColor(generate_color()),
                Print(cli.char)
            )?;
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
    if event::poll(dur)? {
        if let Event::Key(
            KeyEvent {
                code: KeyCode::Char('q'),
                kind: KeyEventKind::Press,
                modifiers: KeyModifiers::NONE,
                ..
            }
            | KeyEvent {
                code: KeyCode::Char('c'),
                kind: KeyEventKind::Press,
                modifiers: KeyModifiers::CONTROL,
                ..
            },
        ) = event::read()?
        {
            return Ok(true);
        }
    }

    Ok(false)
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    terminal::enable_raw_mode()?;

    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen, cursor::Hide)?;

    match cli.mode {
        Mode::FillScreen => fill_screen(&cli, &stdout),
        Mode::Infinite => print_infinite(&cli, &stdout),
        Mode::Random => print_random(&cli, &stdout),
    }?;

    execute!(stdout, LeaveAlternateScreen)?;

    terminal::disable_raw_mode()
}

/// Continuously print `cli.char` at the current cursor position in random colors.
fn print_infinite<W>(cli: &Cli, mut writer: W) -> io::Result<()>
where
    W: Write,
{
    while !is_quitting_char_read(Duration::from_millis(10))? {
        execute!(
            writer,
            SetForegroundColor(generate_color()),
            Print(cli.char)
        )?;
    }

    Ok(())
}

/// Renders a grid of characters, changing the color of a single cell with each iteration.
fn print_random<W>(cli: &Cli, mut writer: W) -> io::Result<()>
where
    W: Write,
{
    let (columns, rows) = terminal::size()?;
    let (u_cols, u_rows) = (usize::from(columns), usize::from(rows));

    let mut grid: Vec<Vec<Option<Color>>> = vec![vec![None; u_rows]; u_cols];

    #[expect(
        clippy::indexing_slicing,
        reason = "grid is guaranteed to be of correct length and height."
    )]
    while !is_quitting_char_read(Duration::from_millis(10))? {
        let (rand_col, rand_row) = (rand::random_range(0..u_cols), rand::random_range(0..u_rows));

        grid[rand_col][rand_row] = Some(generate_color());

        for column in &grid {
            for row in column {
                if let Some(color) = *row {
                    queue!(writer, SetForegroundColor(color), Print(cli.char))?;
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
