use toml;
use std::fs;
use std::fmt;

#[derive(Clone)]
pub enum Source<'a> {
    File(&'a str),
    Memory(&'a str),
}

impl<'a> fmt::Debug for Source<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Source::File(file) => write!(f, "File({:?})", file),
            Source::Memory(data) => {
                write!(
                f,
                "Memory({:?} ..., {})",
                data.chars().take(10).collect::<String>(),
                data.len(),
            )
            }
        }
    }
}

impl<'a> Source<'a> {
    pub(crate) fn try_read(&self) -> Option<toml::Value> {
        match self {
            Source::File(file) => {
                fs::File::open(file)
                    .and_then(|mut f| {
                        use std::io::Read;

                        let mut config_string = String::new();
                        f.read_to_string(&mut config_string).expect(
                            "Reading config file failed after succesful open",
                        );

                        Ok(config_string)
                    })
                    .ok()
                    .map(|s| {
                        s.parse::<toml::Value>().expect(&format!(
                            "File {:?} does not contain valid toml!",
                            file
                        ))
                    })
            }
            Source::Memory(data) => {
                Some(data.parse::<toml::Value>().expect(&format!(
                    "Memory slice does not contain valid toml!"
                )))
            }
        }
    }
}
