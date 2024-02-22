use fern::colors::{Color, ColoredLevelConfig};
use log::LevelFilter;

pub use log::{debug, error, info, trace, warn};

pub struct Logger;

impl Logger {
    pub fn setup_logger() -> Result<(), fern::InitError> {
        let colors = ColoredLevelConfig {
            trace: Color::Cyan,
            debug: Color::Magenta,
            info: Color::Green,
            warn: Color::Red,
            error: Color::BrightRed,
        };

        fern::Dispatch::new()
            .format(move |out, message, record| {
                out.finish(format_args!(
                    "{}[{}] {}",
                    chrono::Local::now().format("[%H:%M:%S]"),
                    colors.color(record.level()),
                    message
                ))
            })
            .chain(std::io::stdout())
            .level(log::LevelFilter::Info)
            .apply()?;

        Ok(())
    }
}
