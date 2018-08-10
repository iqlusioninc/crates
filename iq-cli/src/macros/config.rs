//! Macros for loading and using static configuration values as well as a
//! global configuration value (i.e. `::GLOBAL_CONFIG`).

/// Create a (lazy) static configuration of the given type
#[macro_export]
macro_rules! create_static_config {
    ($static_name:tt, $type:ty) => {
        lazy_static! {
            static ref $static_name: Mutex<RefCell<Option<$type>>> =
                { Mutex::new(RefCell::new(None)) };
        }
    };
}

/// Create global static configuration of the given type.
///
/// This is intended to be called from the crate root (i.e. `main.rs`)
#[macro_export]
macro_rules! create_global_config {
    ($type:ty) => {
        create_static_config!(GLOBAL_CONFIG, $type)
    };
}

/// Get the global configuration static value
macro_rules! global_config_static {
    () => {
        ::GLOBAL_CONFIG
    };
}

/// Set a static configuration to the given value
#[macro_export]
macro_rules! set_static_config {
    ($static_var:expr, $value:expr) => {
        $static_var.lock().unwrap().replace(Some($value));
    };
}

/// Set the global static configuration to the given value
#[macro_export]
macro_rules! set_global_config {
    ($value:expr) => {
        set_static_config!(global_config_static!(), $value)
    };
}

/// Load a static configuration from the TOML file at the given `AsRef<Path>`,
/// printing an error message and exiting if there is an I/O error (e.g. ENOENT)
/// or if the file failed to parse correctly.
#[macro_export]
macro_rules! load_static_config_from_toml {
    ($static_var:expr, $path:expr) => {
        let config = $crate::config::load_toml($path).unwrap_or_else(|e| {
            status_error!("error loading {}: {}", config_file.display(), e);
            exit(1);
        });
        set_static_config!($static_var, config);
    };
}

/// Set the global static configuration to the given value
#[macro_export]
macro_rules! load_global_config_from_toml {
    ($path:expr) => {
        load_static_config_from_toml!(global_config_static!(), $path)
    };
}

/// Lock and borrow a static configuration.
///
/// If no configuration has been loaded, print an error message and exit.
#[macro_export]
macro_rules! get_static_config {
    ($static_var:expr) => {
        let config = $static_var.lock().unwrap();
        config.as_ref().unwrap()
    };
}

/// Lock and borrow the global configuration.
#[macro_export]
macro_rules! get_global_config {
    () => {
        get_static_config!(global_config_static!())
    };
}
