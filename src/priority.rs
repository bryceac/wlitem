pub enum Priority {
    Low,
    Medium,
    High
}

impl Priority {
    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            s if s.to_lowercase() == "low" => Priority::Low,
            s if s.to_lowercase() == "medium" => Priority::Medium,
            s if s.to_lowercase() == "high" => Priority::High,
            _ => None
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Priority::Low => "low",
            priority::Medium => "medium",
            Priority::High => "high"
        }
    }
}