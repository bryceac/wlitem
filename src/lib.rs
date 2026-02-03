mod item;
mod priority;
mod item_parsing_error;

pub use priority::Priority as Priority;
pub use item::Item as Item;
pub use item_parsing_error::ItemParsingError as ItemParsingError;

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

    #[test]
    fn item_creation_throws_error_when_not_enough_fields() {
        let sample_item = Item::from_str("Nintendo Switch\t1\thigh");

        assert_eq!(sample_item.err().unwrap(), ItemParsingError::TooFewFields(3))
    }

    #[test]
    fn item_creation_throws_error_when_too_many_fields() {
        let sample_item = Item::from_str("10\tNintendo Switch\t1\thigh\thttps://example.com\thello");

        assert_eq!(sample_item.err().unwrap(), ItemParsingError::TooManyFields(6))
    }

    #[test]
    fn create_item_via_text() {
        let sample_item = Item::from_str("10\tNintendo Switch\t1\thigh\thttps://example.com");

        assert!(sample_item.is_ok())
    }

    #[test]
    fn item_parses_from_text_correctly() {
        let sample_item = Item::from_str("10\tNintendo Switch\t1\thigh\thttps://example.com");
        let expected_item = Item::from("10", "Nintendo Switch", 1, "high", "https://example.com", vec![]);

        assert_eq!(sample_item.ok().unwrap(), expected_item);
    }

    #[test]
    fn item_priority_defaults_to_medium() {
        let sample_item = Item::from_str("10\tNintendo Switch\t1\t\thttps://example.com");

        assert_eq!(sample_item.ok().unwrap().priority, Priority::Medium)
    }
}
