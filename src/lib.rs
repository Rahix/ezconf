//! ezconf
//! ======
//!
//! A library to add configuration options to your project with as little
//! boilerplate as possible.
//!
//! All macros will cache the value for fast access times (Although, if it is
//! really time critical, you should save it to a variable yourself)
//!
//! **Important**: Due to the way this crate was implemented, it is necessary
//! to import the `lazy_static` macro in your project
//!
//! # Example
//!
//! ```
//! #[macro_use]
//! extern crate lazy_static;
//! #[macro_use]
//! extern crate ezconf;
//!
//! ezconf_file!(CONFIG = "tests/test.toml");
//!
//! fn main() {
//!     let mut value = 100.0f64;
//!     // This is supposed to be a very complex algorithm
//!     for i in 0..1000 {
//!         // The default value (0.1) will be used if the value
//!         // does not exist in the config
//!         let CONSTANT = ezconf_float!(CONFIG: "float.a", 0.1);
//!
//!         value = value.powf(CONSTANT);
//!     }
//! }
//! ```
#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces, unused_qualifications)]

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

pub extern crate toml;
pub extern crate toml_query;

/// Open a config file and store it in a static variable for easy access later on
///
/// # Example
///
/// ```
/// # #[macro_use]
/// # extern crate lazy_static;
/// # #[macro_use]
/// # extern crate ezconf;
/// ezconf_file!(CONFIG = "tests/test.toml");
///
/// # fn main() {
/// #   CONFIG.get("integer").unwrap();
/// # }
/// ```
#[macro_export]
macro_rules! ezconf_file {
    ($confname:ident = $file:expr) => (
        lazy_static! {
            static ref $confname: $crate::toml::Value = {
                use ::std::io::Read;
                let mut cfg_string = String::new();
                ::std::fs::File::open($file)
                    .expect("Can't read config file!")
                    .read_to_string(&mut cfg_string)
                    .expect("Can't read config file to string!");


                cfg_string.parse::<$crate::toml::Value>()
                    .expect("Can't parse config file!")
            };
        }
    )
}

/// Read a `toml::Value` from the config
///
/// # Example
///
/// ```
/// # #[macro_use]
/// # extern crate lazy_static;
/// # #[macro_use]
/// # extern crate ezconf;
/// extern crate toml;
///
/// ezconf_file!(CONFIG = "tests/test.toml");
///
/// # fn main() {
/// let val = ezconf_toml!(CONFIG: "integer", toml::Value::Boolean(false));
/// assert!(val.is_table());
/// # }
/// ```
#[macro_export]
macro_rules! ezconf_toml {
    ($confname:ident: $key:expr, $default:expr) => ({
        lazy_static! {
            static ref DEFAULT: $crate::toml::Value = {
                $default
            };
            static ref CONTAINER: &'static $crate::toml::Value = {
                use $crate::toml_query::read::TomlValueReadExt;
                $confname.read($key)
                    .expect("Reading from config failed")
                    .unwrap_or(&DEFAULT)
            };
        }

        *CONTAINER
    })
}

/// Read a string (`&str`) from the config
///
/// # Example
///
/// ```
/// # #[macro_use]
/// # extern crate lazy_static;
/// # #[macro_use]
/// # extern crate ezconf;
/// ezconf_file!(CONFIG = "tests/test.toml");
///
/// # fn main() {
/// let val = ezconf_str!(CONFIG: "string.a", "Default");
/// assert!(val == "Foo");
/// # }
/// ```
#[macro_export]
macro_rules! ezconf_str {
    ($confname:ident: $key:expr, $default:expr) => ({
        lazy_static! {
            static ref CONTAINER: &'static str = {
                use $crate::toml_query::read::TomlValueReadExt;
                $confname.read($key)
                    .expect("Reading from config failed")
                    .map(|v| v.as_str().expect(&format!("'{}' should be a string", $key)))
                    .unwrap_or($default)
            };
        }

        *CONTAINER
    })
}

/// Read an integer (`i64`) from the config
///
/// # Example
///
/// ```
/// # #[macro_use]
/// # extern crate lazy_static;
/// # #[macro_use]
/// # extern crate ezconf;
/// ezconf_file!(CONFIG = "tests/test.toml");
///
/// # fn main() {
/// let val = ezconf_int!(CONFIG: "integer.b", 123);
/// assert!(val == 23434248);
/// # }
/// ```
#[macro_export]
macro_rules! ezconf_int {
    ($confname:ident: $key:expr, $default:expr) => ({
        lazy_static! {
            static ref CONTAINER: i64 = {
                use $crate::toml_query::read::TomlValueReadExt;
                $confname.read($key)
                    .expect("Reading from config failed")
                    .map(|v| v.as_integer().expect(&format!("'{}' should be an integer", $key)))
                    .unwrap_or($default)
            };
        }

        *CONTAINER
    })
}

/// Read a float (`f64`) from the config
///
/// # Example
///
/// ```
/// # #[macro_use]
/// # extern crate lazy_static;
/// # #[macro_use]
/// # extern crate ezconf;
/// ezconf_file!(CONFIG = "tests/test.toml");
///
/// # fn main() {
/// let val = ezconf_float!(CONFIG: "float.a", 123.456);
/// assert!((val - 2.0f64.sqrt()) < 0.0001);
/// # }
/// ```
#[macro_export]
macro_rules! ezconf_float {
    ($confname:ident: $key:expr, $default:expr) => ({
        lazy_static! {
            static ref CONTAINER: f64 = {
                use $crate::toml_query::read::TomlValueReadExt;
                $confname.read($key)
                    .expect("Reading from config failed")
                    .map(|v| v.as_float().expect(&format!("'{}' should be a float", $key)))
                    .unwrap_or($default)
            };
        }

        *CONTAINER
    })
}

/// Read a boolean from the config
///
/// # Example
///
/// ```
/// # #[macro_use]
/// # extern crate lazy_static;
/// # #[macro_use]
/// # extern crate ezconf;
/// ezconf_file!(CONFIG = "tests/test.toml");
///
/// # fn main() {
/// let val1 = ezconf_bool!(CONFIG: "boolean.available", false);
/// let val2 = ezconf_bool!(CONFIG: "boolean.unavailable", false);
/// assert!(val1);
/// assert!(!val2);
/// # }
/// ```
#[macro_export]
macro_rules! ezconf_bool {
    ($confname:ident: $key:expr, $default:expr) => ({
        lazy_static! {
            static ref CONTAINER: bool = {
                use $crate::toml_query::read::TomlValueReadExt;
                $confname.read($key)
                    .expect("Reading from config failed")
                    .map(|v| v.as_bool().expect(&format!("'{}' should be a boolean", $key)))
                    .unwrap_or($default)
            };
        }

        *CONTAINER
    })
}

#[cfg(test)]
mod test {
    use toml;

    ezconf_file!(CONFIG = "./Cargo.toml");

    #[test]
    fn test_config_available() {
        CONFIG.get("package").unwrap();
    }

    #[test]
    fn test_ezconf_toml() {
        assert!(
            ezconf_toml!(CONFIG: "package.name", toml::Value::String("default".into())) ==
                &toml::Value::String("ezconf".into())
        );
    }
}
