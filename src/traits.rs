use crate::prelude::*;

pub trait Schema {
    fn new() -> Self;
    fn add_property(&mut self, name: String, value: String) -> Result<(), Error>;
    fn add_item(&mut self, name: String, item: Types) -> Result<(), Error>;
}

pub enum Error {
    InvalidProperty,
    InvalidType,
    InvalidValue,
}