mod item;
mod priority;

pub use priority::Priority as Priority;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_enum_is_none() {
        let invalid_data = Priority::from_str("hello");

        assert_eq!(invalid_data, None)
    }
}
