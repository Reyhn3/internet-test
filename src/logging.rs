use std::cell::RefCell;
use anstream::println;
use anstyle::{Ansi256Color, Color, Style};
use chrono::Local;
use log::{debug, error, info, log_enabled, trace, warn, Level, LevelFilter};
use anstyle::AnsiColor::{Black, White};

const DATE_FORMAT_STR: &'static str = "%H:%M:%S.%f";
static TIMESTAMP_STYLE: Style = Style::new()
    .fg_color(Some(Color::Ansi256(Ansi256Color(242))));
static LEVEL_STYLE: Style = Style::new()
    .bg_color(Some(Color::Ansi(White)))
    .fg_color(Some(Color::Ansi(Black)));
static SCOPE_STYLE: Style = Style::new()
    .fg_color(Some(Color::Ansi256(Ansi256Color(242))));

thread_local! {
    static SCOPE_ID: RefCell<Option<usize>> = RefCell::new(None);
}

pub fn set_scope(id: usize) {
    SCOPE_ID.with(|scope| {
        *scope.borrow_mut() = Some(id);
    });
}

pub fn clear_scope() {
    SCOPE_ID.with(|scope| {
        *scope.borrow_mut() = None;
    });
}

pub fn init(quiet: bool, verbose: bool) {
    if quiet {
        env_logger::builder()
            .filter_level(LevelFilter::Off)
            .init();
        return;
    }

    if !verbose {
        env_logger::builder()
            .format(|buf, record| {
                let style = buf.default_level_style(record.level());
                println!("{style}{}{style:#}", record.args());
                Ok(())
            })
            .filter_level(LevelFilter::Info)
            .init();
        return;
    }

    env_logger::builder()
        .format(|buf, record| {
            let style = buf.default_level_style(record.level());
            let timestamp = Local::now().format(DATE_FORMAT_STR);
            let pad = match record.level() {
                Level::Info | Level::Warn => "  ",
                _ => " ",
            };

            let scope = SCOPE_ID.with(|s| *s.borrow());
            let scope_prefix = match scope {
                Some(id) => format!("[{}] ", id),
                None => "".to_string(),
            };

            println!(
                "{TIMESTAMP_STYLE}{timestamp}{TIMESTAMP_STYLE:#} {LEVEL_STYLE}{}{LEVEL_STYLE:#}{pad}{SCOPE_STYLE}{scope_prefix}{SCOPE_STYLE:#}{style}{}{style:#}",
                record.level(),
                record.args()
            );
            Ok(())
        })
        .filter_level(LevelFilter::max())
        .format_target(false)
        .init();
}

//TODO: Remove when done learning
pub fn log_debug(debug: bool) {
    if !cfg!(debug_assertions) {
        return;
    }

    if !debug {
        return;
    }

    error!("{}", "Its fleece was white as snow");
    warn!("{:#?}", "The lamb was sure to go");
    info!("{:?}", "And every where that Mary went");
    debug!("Mary has a little lamb");
    trace!("Mary has a fluffy lamb");

    debug!("this is a debug {}", "message");
    error!("this is printed by default");

    if log_enabled!(Level::Info) {
        let x = 3 * 4; // expensive computation
        info!("the answer was: {}", x);
    }
}
