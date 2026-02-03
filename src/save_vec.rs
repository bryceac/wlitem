use crate::Item;

use std::{fs::File, io::{ Write, Error } };

use serde_json;
pub trait Save {
    fn save(&self, path: &str) -> Result<(), Error>;
}

impl Save for Vec<Item> {
    fn save(&self, path: &str) -> Result<(), Error> {
        let mut output = File::create(path)?;
        let json_sring = serde_json::to_string_pretty(self)?;

        match write!(output, "{}", json_sring) {
            Ok(()) => Ok(()),
            Err(error) => Err(error)
        }
    }
}