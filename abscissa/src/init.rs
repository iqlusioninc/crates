//! `abscissa` framework initialization

#[cfg(feature = "simplelog")]
use simplelog::{self, CombinedLogger, LevelFilter, TermLogger};

use shell::{self, ColorConfig};

/// `abscissa` framework options which can be supplied when initializing
#[derive(Debug)]
pub struct InitOpts {
    /// Color configuration
    pub color_config: ColorConfig,

    /// Enable logging subsystem
    #[cfg(feature = "simplelog")]
    pub enable_logging: bool,

    /// Enable verbose logging
    pub verbose: bool,
}

/// Framework default options
impl Default for InitOpts {
    fn default() -> InitOpts {
        InitOpts {
            color_config: ColorConfig::default(),
            #[cfg(feature = "simplelog")]
            enable_logging: true,
            verbose: false,
        }
    }
}

/// Initialize a command-line app with the given options
pub fn init<I: Into<InitOpts>>(into_opts: I) {
    let opts = into_opts.into();

    // Initialize the shell
    shell::config(opts.color_config);

    // Initialize the logging subsystem
    #[cfg(feature = "simplelog")]
    {
        if opts.enable_logging {
            init_logging(Default::default(), opts.verbose);
        }
    }
}

/// Initialize the logging subsystem (i.e. simplelog)
#[cfg(feature = "simplelog")]
fn init_logging(config: simplelog::Config, verbose: bool) {
    let level_filter = if verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    if let Some(logger) = TermLogger::new(level_filter, config) {
        CombinedLogger::init(vec![logger]).unwrap()
    }
}
