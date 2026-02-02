#[derive(Clone, Debug, PartialEq, Eq, PartialOrd)]
pub enum Priority {
    Low,
    Medium,
    High
}

impl Priority {
    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            s if s.to_lowercase() == "low" => Some(Priority::Low),
            s if s.to_lowercase() == "medium" => Some(Priority::Medium),
            s if s.to_lowercase() == "high" => Some(Priority::High),
            _ => None
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Priority::Low => "low",
            Priority::Medium => "medium",
            Priority::High => "high"
        }
    }
}