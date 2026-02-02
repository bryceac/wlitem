use uuid::Uuid;

use crate::Priority;

use url::Url;

/// structure that represents an item in a wishlist

#[derive(Clone, PartialEq, Eq)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub quantity: u32,
    pub priority: Priority,
    pub url: Option<Url>,
    pub notes: Vec<String>
}

impl Item {
    /**
     * This method creates a builder that will help facilate in creating
     * a wish list item.
     * 
     * Items can then be created like this:
     * 
     * let nintendo_switch_2 = Item::builder()
     * .set_name("Nintendo Switch 2")
     * .set_quantity(1)
     * .set_priority("high")
     * .set_url("https://example.com/nintendo-switch-2")
     * .build();
     */
    pub fn builder() -> ItemBuilder {
        ItemBuilder::new()
    }

    /// this method utilizes the builder method to create an an empty item.
    pub fn new() -> Self {
        Item::builder().build()
    }

    pub fn to_string(&self) -> String {
        let mut content = format!("{}\t{}\t{}\t{}\t{}\r\n\r\n", 
        self.id, 
        self.name, 
        self.quantity, 
        self.priority.to_str(), 
        if let Some(ref url) = self.url {
            url.as_str()
        } else {
            ""
        });

        content.push_str("-----\r\n");

        for note in self.notes.clone() {
            let formatted_note = format!("{}\r\n\r\n", note);

            content.push_str(&formatted_note);
        }

        content
    }
}

pub struct ItemBuilder {
    pub id: String,
    pub name: Option<String>,
    pub quantity: Option<u32>,
    pub priority: Option<Priority>,
    pub url: Option<Url>,
    pub notes: Vec<String>
}

impl ItemBuilder {
    pub fn new() -> Self {
        ItemBuilder {
            id: Uuid::new_v4().hyphenated().encode_upper(&mut Uuid::encode_buffer()).to_string(),
            name: None,
            quantity: None,
            priority: None,
            url: None,
            notes: vec![]
        }
    }

    /**
     * set the identifier of an item.
     * If this is unused, a default value will be provided.
     */
    pub fn set_id(&mut self, id: &str) -> &mut Self {
        self.id = id.to_owned();
        self
    }

    pub fn set_name(&mut self, name: &str) -> &mut Self {
        self.name = Some(name.to_owned());
        self
    }

    pub fn set_quantity(&mut self, quantity: u32) -> &mut Self {
        self.quantity = Some(quantity);
        self
    }

    pub fn set_priority(&mut self, priority: &str) -> &mut Self {
        self.priority = Priority::from_str(priority);
        self
    }

    /**
     * sets the URL. If the URL is not valid for any reason, it will
     * set the value to none.
     */
    pub fn set_url(&mut self, url: &str) -> &mut Self {
        self.url = if let Ok(url) = Url::parse(url) {
            Some(url)
        } else {
            None
        };

        self
    }

    pub fn add_note(&mut self, note: &str) -> &mut Self {
        self.notes.push(note.to_owned());
        self
    }

    /**
     * this method is used to actually build the wishlist item.
     * any items not provided a value with the respective set methods
     * will be given a default value.
     */
    pub fn build(&self) -> Item {
        Item {
            id: self.id.clone(),
            name: if let Some(ref name) = self.name {
                name.to_owned()
            } else {
                String::default()
            },
            quantity: if let Some(ref quantity) = self.quantity {
                quantity.to_owned()
            } else {
                1
            },
            priority: if let Some(ref priority) = self.priority {
                priority.clone()
            } else {
                Priority::Medium
            },
            url : self.url.clone(),
            notes: self.notes.clone()
        }
    }
}