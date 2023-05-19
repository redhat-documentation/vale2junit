use color_eyre::eyre::{Context, Result};
use simplelog::{ColorChoice, ConfigBuilder, LevelFilter, TermLogger, TerminalMode};

/// This function initializes the `simplelog` logging system, which plugs into the `log`
/// infrastructure. The function returns nothing. It only affects the global state when it runs.
pub fn initialize_logger(verbosity: usize) -> Result<()> {
    // Set the verbosity level based on the command-line options.
    let verbosity = match verbosity {
        // The default verbosity level is Warn.
        0 => LevelFilter::Warn,
        1 => LevelFilter::Info,
        _ => LevelFilter::Debug,
    };

    let config = ConfigBuilder::new()
        // Display a time stamp only for the most verbose level.
        .set_time_level(LevelFilter::Debug)
        // Display the thread number only for the most verbose level.
        .set_thread_level(LevelFilter::Debug)
        .build();

    TermLogger::init(
        verbosity,
        config,
        // Log everything to stderr.
        TerminalMode::Stderr,
        // Try to use color if possible.
        ColorChoice::Auto,
    )
    .context("Failed to configure the terminal logging.")?;

    Ok(())
}
