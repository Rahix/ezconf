#[macro_use]
extern crate log;
extern crate once_cell;
pub extern crate toml;
extern crate toml_query;

pub mod config;
pub use config::{Config, INIT};

pub mod source;
pub use source::Source;
