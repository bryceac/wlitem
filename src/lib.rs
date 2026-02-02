mod item;
mod priority;

pub use priority::Priority as Priority;
pub use item::Item as Item;

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

    #[test]
    fn enum_parses_correctly() {
        let enum_value = Priority::from_str("low").unwrap();

        assert_eq!(enum_value, Priority::Low)
    }

    #[test]
    fn url_is_none_if_not_valid() {
        let sample_item = Item::builder()
        .set_name("Sample Item")
        .set_url("")
        .build();

        assert_eq!(sample_item.url, None)
    }
}
