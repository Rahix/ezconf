#[cfg(test)]
#[macro_use]
extern crate lazy_static;

pub extern crate toml;
pub extern crate toml_query;

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
        assert!(ezconf_toml!(CONFIG: "package.name", toml::Value::String("default".into()))
          == &toml::Value::String("ezconf".into())
        );
    }
}
