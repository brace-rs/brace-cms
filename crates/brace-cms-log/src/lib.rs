use chrono::Local;
use fern::colors::ColoredLevelConfig;
use fern::{log_file, Dispatch, InitError};

use brace_config::Config;

use self::level::Level;
use self::output::Output;

pub use log::{debug, error, info, log, trace, warn};

pub mod level;
pub mod output;

pub fn init(config: &Config) -> Result<(), InitError> {
    let level: Level = config.get("log.level").unwrap_or_default();
    let output: Output = config.get("log.output").unwrap_or_default();
    let mut logger = Dispatch::new();

    logger = match output {
        Output::File(_) => logger.format(move |out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                Local::now().format("%Y-%m-%dT%H:%M:%SZ"),
                record.level(),
                record.target(),
                message,
            ))
        }),
        _ => {
            let colors = ColoredLevelConfig::new();

            logger.format(move |out, message, record| {
                out.finish(format_args!(
                    "[{} {} {}] {}",
                    Local::now().format("%Y-%m-%dT%H:%M:%SZ"),
                    colors.color(record.level()),
                    record.target(),
                    message,
                ))
            })
        }
    };

    logger = logger.level(level.into());
    logger = match output {
        Output::Stdout => logger.chain(std::io::stdout()),
        Output::Stderr => logger.chain(std::io::stderr()),
        Output::File(file) => logger.chain(log_file(file)?),
    };
    logger.apply()?;

    Ok(())
}
