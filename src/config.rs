use once_cell;
use toml;
use std::iter;
use source;

#[derive(Debug)]
pub struct Config(once_cell::sync::OnceCell<toml::Value>);
pub const INIT: Config = Config(once_cell::sync::OnceCell::INIT);

impl Config {
    pub fn init<'a>(
        &self,
        sources: impl iter::Iterator<Item = &'a source::Source<'a>>,
    ) -> Result<bool, ()> {
        let value = sources
            .inspect(|s| info!("Trying config source {:?} ...", s))
            .filter_map(|s| s.try_read().map(|v| (v, true)))
            .inspect(|_| info!("Config loaded!"))
            .next()
            .unwrap_or_else(|| {
                info!("Using empty (default) config!");
                (toml::Value::Table(toml::value::Table::new()), false)
            });

        let res = value.1;
        self.0.set(value.0)
            .map(|()| res)
            .map_err(|_: toml::Value| {
                error!("Failed setting config cell. Maybe it was already initialized?");
            })
    }

    pub fn get<'a, T: toml::macros::Deserialize<'a> + ::std::fmt::Debug>(
        &self,
        path: &str,
    ) -> Option<T> {
        use toml_query::read::TomlValueReadExt;

        self.0
            .get()
            .unwrap_or_else(|| {
                error!("Failed opening config cell! Maybe it isn't initialized yet?");
                panic!("Failed to open config cell");
            })
            .read(path)
            .unwrap_or_else(|_| {
                error!("Reading config value {:?} failed!", path);
                panic!("Reading config value {:?} failed!", path);
            })
            .map(|v| {
                v.clone().try_into::<T>().unwrap_or_else(|e| {
                    error!("Failed parsing value {:?}: {:?}", path, e);
                    panic!("Failed parsing value {:?}: {:?}", path, e);
                })
            })
            .map(|v| {
                debug!("{:?}: {:?}", path, v);
                v
            })
    }

    pub fn get_or<'a, T: toml::macros::Deserialize<'a> + ::std::fmt::Debug>(
        &self,
        path: &str,
        def: T,
    ) -> T {
        self.get::<T>(path).unwrap_or_else(|| {
            debug!("{:?}: {:?} (default)", path, def);

            def
        })
    }
}
