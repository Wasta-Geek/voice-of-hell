use simplelog::{ColorChoice, Config, LevelFilter, TermLogger, TerminalMode};

/// Init logger
///
/// # Panics
///
/// Will panic if logger cannot be init (called multiples times probably)
pub fn init_logger() {
    let log_level = match cfg!(debug_assertions) {
        true => LevelFilter::Debug,
        false => LevelFilter::Warn,
    };
    TermLogger::init(
        log_level,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .expect("Cannot instantiate logger properly");
}
