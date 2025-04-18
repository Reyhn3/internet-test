use chrono::Local;
use log::{debug, error, info, log_enabled, trace, warn, Level, LevelFilter};
use std::io::Write;

const DATE_FORMAT_STR: &'static str = "%H:%M:%S";

pub fn init(quiet: bool, verbose: bool) {
    if quiet {
        env_logger::builder().filter_level(LevelFilter::Off).init();
        return;
    }

    if !verbose {
        env_logger::builder()
            .format(|buf, record| {
                let style = buf.default_level_style(record.level());
                writeln!(buf, "{style}{}{style:#}", record.args())
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

            writeln!(
                buf,
                "{timestamp} [{}]{pad}{style}{}{style:#}",
                record.level(),
                record.args()
            )
        })
        .filter_level(LevelFilter::max())
        .format_target(false)
        .init();
}

//TODO: Remove
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
