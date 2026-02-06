use std::str::FromStr;

use serde::{ Serialize, Deserialize };

use crate::priority_parse_error::PriorityParseError;

/// variants for priority levels
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd)]
#[serde(rename_all = "lowercase")]
pub enum Priority {
    Low,
    Medium,
    High,
    Highest
}

impl Priority {
    pub fn to_str(&self) -> &str {
        match self {
            Priority::Low => "low",
            Priority::Medium => "medium",
            Priority::High => "high",
            Priority::Highest => "highest"
        }
    }

    pub fn is_low(&self) -> bool {
        self.clone() == Priority::Low
    }

    pub fn is_medium(&self) -> bool {
        self.clone() == Priority::Medium
    }

    pub fn is_high(&self) -> bool {
        self.clone() == Priority::High
    }
}

impl FromStr for Priority {
    type Err = PriorityParseError;
    
    fn from_str(value: &str) -> Result<Self, PriorityParseError> {
        match value {
            s if s.to_lowercase() == "low" => Ok(Priority::Low),
            s if s.to_lowercase() == "medium" => Ok(Priority::Medium),
            s if s.to_lowercase() == "high" => Ok(Priority::High),
            s if s.to_lowercase() == "highest" => Ok(Priority::Highest),
            _ => Err(PriorityParseError::InvalidPriority(value.to_owned()))
        }
    }
}