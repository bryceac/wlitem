use std::{ error::Error, fmt };

#[derive(Debug)]
pub enum PriorityParseError {
    InvalidPriority(String)
}

impl fmt::Display for PriorityParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidPriority(s) => {
                let error_string = format!("{} is not a valid priority level", s);

                write!(f, "{}", error_string)
            }
        }
    }
}

impl Error for PriorityParseError {}