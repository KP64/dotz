use clap::{Parser, ValueEnum};
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute, queue,
    style::{Color, Print, SetForegroundColor},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen, size},
};
use itertools::iproduct;
use std::{
    fmt,
    io::{self, Write},
    time::Duration,
};

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

#[derive(Debug, Default, Clone, ValueEnum)]
enum Mode {
    FillScreen,

    #[default]
    Infinite,

    Random,
}

impl fmt::Display for Mode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let str = match self {
            Self::FillScreen => "fill-screen",
            Self::Infinite => "infinite",
            Self::Random => "random",
        };
        write!(fmt, "{str}")
    }
}

fn fill_screen(cli: &Cli, mut writer: impl Write) -> io::Result<()> {
    let (cols, rows) = size()?;

    for _ in iproduct!(0..cols, 0..rows) {
        queue!(
            writer,
            SetForegroundColor(generate_color()),
            Print(cli.char)
        )?;
    }

    writer.flush()?;

    while !is_quitting_char_read(Duration::MAX)? {}

    Ok(())
}

#[must_use]
fn generate_color() -> Color {
    Color::AnsiValue(rand::random_range(0..8))
}

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

fn print_infinite(cli: &Cli, mut writer: impl Write) -> io::Result<()> {
    while !is_quitting_char_read(Duration::from_millis(10))? {
        execute!(
            writer,
            SetForegroundColor(generate_color()),
            Print(cli.char)
        )?;
    }

    Ok(())
}

fn print_random(cli: &Cli, mut writer: impl Write) -> io::Result<()> {
    let (columns, rows) = terminal::size()?;
    let (columns, rows) = (usize::from(columns), usize::from(rows));

    let mut grid: Vec<Vec<Option<Color>>> = vec![vec![None; rows]; columns];

    while !is_quitting_char_read(Duration::from_millis(10))? {
        let (column, row) = (rand::random_range(0..columns), rand::random_range(0..rows));

        grid[column][row] = Some(generate_color());

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

    #[test]
    fn verify_cli() {
        Cli::command().debug_assert();
    }
}
