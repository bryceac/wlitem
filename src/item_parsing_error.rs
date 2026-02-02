use core::fmt;

#[derive(Debug, PartialEq)]
pub enum ItemParsingError {
    TooFewFields(usize),
    TooManyFields(usize)
}

impl ItemParsingError {
    pub fn to_string(&self) -> String {
        match self {
            ItemParsingError::TooFewFields(fields) => format!("Item contains {} fields, which is not enough. Items MUST contain 5 fields.", fields),
            ItemParsingError::TooManyFields(fields) => format!("Item contains {} fields, which is too many. Items MUST contain 5 fields.", fields)
        }
    }
}

impl fmt::Display for ItemParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}