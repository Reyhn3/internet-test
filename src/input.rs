use clap::{builder, Parser};
use std::fmt::Debug;
use anstyle::{AnsiColor, Color, Style};

/// Checks whether there is a working Internet connection.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[command(styles=get_styles())]
pub struct Args {
//TODO: Remove when done developing
    /// Enable debug logs to the terminal.
    #[clap(long, short, action, hide(true))]
    pub(crate) debug: bool,

//TODO: Remove when done developing
    /// When set, checks will throw errors
    #[clap(long, short, action, hide(true))]
    pub(crate) error: bool,
    
    /// Disable all output to the terminal.
    #[clap(long, short, action)]
    pub(crate) quiet: bool,

    /// Enable full output to the terminal.
    #[clap(long, short, action)]
    pub(crate) verbose: bool
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
        .literal(
            Style::new().fg_color(Some(Color::Ansi(AnsiColor::Green))),
        )
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
        .placeholder(
            Style::new().fg_color(Some(Color::Ansi(AnsiColor::White))),
        )
}
