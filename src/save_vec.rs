use crate::Item;

use std::{fs::File, io::{ Write, Error } };

use serde_json;
pub trait Save {
    fn save(&self, path: &str) -> Result<(), Error>;
    fn save_tsv(&self, path: &str) -> Result<(), Error>;
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

    fn save_tsv(&self, path: &str) -> Result<(), Error> {
        let mut items = String::new();

        for item in self {
            let item_string = format!("{}\t{}\t{}\t{}\t{}\r\n", 
            item.id, 
            item.name, 
            item.quantity, 
            item.priority.to_str(), 
            if let Some(ref url) = item.url {
                url.as_str()
            } else {
                ""
            });

            items.push_str(&item_string);
        }

        let mut output = File::create(path)?;

        match write!(output, "{}", items) {
            Ok(()) => Ok(()),
            Err(error) => Err(error)
        }
    }
}