use anstyle::{AnsiColor, Color, Style};
use clap::{builder, Parser};
use std::fmt::Debug;

use crate::check_connectivity::checks::Target;

/// Checks whether there is a working Internet connection.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[command(styles=get_styles())]
pub struct Args {
    /// Connectivity target to use for the connectivity check.
    #[clap(long, short, value_enum, default_value_t = Target::Ncsi)]
    pub(crate) target: Target,

    /// Enable debug logs to the terminal.
    #[clap(long, short, action, hide(true))]
    pub(crate) debug: bool,

    /// Disable all output to the terminal.
    #[clap(long, short, action)]
    pub(crate) quiet: bool,

    /// Enable full output to the terminal.
    #[clap(long, short, action)]
    pub(crate) verbose: bool,
}

fn get_styles() -> builder::Styles {
    builder::Styles::styled()
        .usage(
            Style::new()
                .bold()
                .underline()
                .fg_color(Some(Color::Ansi(AnsiColor::Yellow))),
        )
        .header(
            Style::new()
                .bold()
                .underline()
                .fg_color(Some(Color::Ansi(AnsiColor::Yellow))),
        )
        .literal(Style::new().fg_color(Some(Color::Ansi(AnsiColor::Green))))
        .invalid(
            Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Red))),
        )
        .error(
            Style::new()
                .bold()
                .fg_color(Some(Color::Ansi(AnsiColor::Red))),
        )
        .valid(
            Style::new()
                .bold()
                .underline()
                .fg_color(Some(Color::Ansi(AnsiColor::Green))),
        )
        .placeholder(Style::new().fg_color(Some(Color::Ansi(AnsiColor::White))))
}
