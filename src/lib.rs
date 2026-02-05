//! Dotz Utils

use clap::{Args, Parser, Subcommand};
use crossterm::{style::Color, terminal};
use std::{fmt, io, time::Duration};

/// The main command line Parser
#[derive(Parser)]
#[command(version, about, long_about = None, propagate_version = true)]
pub struct Cli {
    /// The character to be printed
    #[arg(short, long, default_value_t = '.', global = true)]
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
#[derive(Default, Subcommand, Debug, Clone)]
pub enum Mode {
    /// Fill the screen immediately
    #[default]
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

    /// Print a separator character every few spaces
    Spaced {
        /// The Speed in which to print
        /// the characters
        #[command(flatten)]
        speed: Speed,

        /// The separator character that is
        /// printed between every few spaces
        #[arg(short = 'S', long, default_value_t = '*')]
        separator: char,

        /// The amount of characters between separator characters
        #[arg(short, long, default_value_t = 3, value_parser = clap::value_parser!(u16).range(1..))]
        spaces: u16,
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
///
/// # Errors
///
/// If no tty is detected
pub fn terminal_area_size() -> io::Result<usize> {
    terminal::size().map(|(cols, rows)| usize::from(cols).saturating_mul(usize::from(rows)))
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
