//! Dotz Utils

use clap::{Args, Parser, Subcommand, arg};
use crossterm::{style::Color, terminal};
use std::{fmt, time::Duration};

/// The default character to be
/// printed in any case.
const DEFAULT_CHAR: char = '.';

/// The main command line Parser
#[derive(Parser)]
#[command(version, about, long_about = None, propagate_version = true)]
pub struct Cli {
    /// The character to be printed
    #[arg(short, long, default_value_t = DEFAULT_CHAR, global = true)]
    pub char: char,

    /// The printing mode
    #[command(subcommand)]
    pub mode: Option<Mode>,
}

/// The Speed in which to print
/// the characters
#[derive(Args, Debug, Clone)]
pub struct Speed {
    /// The speed as "iterations per second"
    #[arg(short, long, default_value_t = 240.0)]
    pub ips: f64,
}

/// Available printing modes.
#[derive(Subcommand, Debug, Clone)]
pub enum Mode {
    /// Fill the screen immediately
    FillScreen,

    /// Continuously print characters
    Infinite {
        /// The speed in which to print
        /// the characters
        #[command(flatten)]
        speed: Speed,
    },

    /// Randomly color individual cells over time
    Random {
        /// The Speed in which to print
        /// the characters
        #[command(flatten)]
        speed: Speed,
    },

    /// Print a Character every few spaces
    Spaced {
        /// The Speed in which to print
        /// the characters
        #[command(flatten)]
        speed: Speed,

        /// The separator character that is
        /// printed between each character to print
        #[arg(short = 'S', long, default_value_t = DEFAULT_CHAR)]
        separator: char,

        /// The amount of separator characters
        /// between each character to print
        #[arg(short, long, default_value_t = 3, value_parser = valid_spacing)]
        spaces: usize,
    },
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match *self {
            Self::FillScreen => "fill-screen",
            Self::Infinite { .. } => "infinite",
            Self::Random { .. } => "random",
            Self::Spaced { .. } => "spaced",
        };

        f.write_str(str)
    }
}

/// Generate a random ANSI Xterm system color.
/// This is done to fully support Xterm and increase usability.
///
/// # Returns
///
/// A `Color::AnsiValue` in the range 0..16.
#[must_use]
pub fn generate_ansi_color() -> Color {
    Color::AnsiValue(rand::random_range(0..16))
}

/// Returns the input polling duration
///
/// # Errors
///
/// When ips is 0
pub fn get_duration(ips: f64) -> Result<Duration, String> {
    let key_wait_dur = 1.0 / ips;
    Duration::try_from_secs_f64(key_wait_dur).map_err(|_err| format!("{ips} ips is invalid"))
}

/// Gets the terminal area
///
/// # Returns
///
/// the terminal area in pixels (width * height)
#[expect(
    clippy::missing_panics_doc,
    reason = "No need for docs, as it shouldn't panic"
)]
#[must_use]
pub fn terminal_area_size() -> usize {
    #[expect(clippy::unwrap_used, reason = "Shouldn't really panic")]
    let (cols, rows) = terminal::size().unwrap();
    usize::from(cols).saturating_mul(usize::from(rows))
}

/// Check for the Spaced mode, whether it makes sense
/// for the user to use that mode or fall back to the
/// infinite mode.
fn valid_spacing(input: &str) -> Result<usize, String> {
    let spaces = input
        .parse::<usize>()
        .map_err(|_err| format!("`{input}` is not a valid number"))?;
    if spaces == 0 {
        Err("You should use the `infinite` mode if you don't want any spaces".to_owned())
    } else {
        Ok(spaces)
    }
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
