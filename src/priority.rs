use serde::{ Serialize, Deserialize };

/// variants for priority levels
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd)]
pub enum Priority {
    Low,
    Medium,
    High,
    Highest
}

impl Priority {
    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            s if s.to_lowercase() == "low" => Some(Priority::Low),
            s if s.to_lowercase() == "medium" => Some(Priority::Medium),
            s if s.to_lowercase() == "high" => Some(Priority::High),
            s if s.to_lowercase() == "highest" => Some(Priority::Highest),
            _ => None
        }
    }

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